package handler

import (
	"fmt"
	"time"

	"github.com/invopop/ctxi18n"
	"github.com/invopop/ctxi18n/i18n"
	"git.at.oechsler.it/samuel/dash/v2/app/command"
	"git.at.oechsler.it/samuel/dash/v2/app/query"
	webi18n "git.at.oechsler.it/samuel/dash/v2/delivery/web/i18n"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/middleware"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/templ/layout"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/templ/page"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/templ/partials"
	"git.at.oechsler.it/samuel/dash/v2/infra/oidc"

	"github.com/gofiber/fiber/v3"
)

const (
	DashboardRoute                      = "DashboardRoute"
	DashboardGreetingRoute              = "DashboardGreetingRoute"
	DashboardTitleApplicationsRoute     = "DashboardTitleApplicationsRoute"
	DashboardTitleApplicationsEditRoute = "DashboardTitleApplicationsEditRoute"
	DashboardTitleBookmarksRoute        = "DashboardTitleBookmarksRoute"
	DashboardTitleBookmarksEditRoute    = "DashboardTitleBookmarksEditRoute"
	DashboardEditRoute                  = "DashboardEditRoute"
	DashboardModalCloseRoute            = "DashboardModalCloseRoute"
)

type DashboardDeps struct {
	SessionStore       *oidc.SessionStore
	App                *fiber.App
	GetUserDashboard   query.UserDashboardGetter
	GetUserSettings    query.UserSettingsGetter
	EnsureDefaultTheme command.DefaultThemeEnsurer
	GetUserThemeByID   query.UserThemeByIDGetter
}

func Dashboard(deps DashboardDeps) {
	router := deps.App.
		Group("/").
		Use(middleware.LoadUserFromSession(deps.SessionStore))

	router.Get("/", func(c fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		// Trigger dashboard auto-provisioning if needed (also ensures settings exist).
		if _, err := deps.GetUserDashboard.Handle(
			c.Context(),
			user.UserID,
			user.Groups,
			user.FirstName,
			time.Time{},
		); err != nil {
			return err
		}

		settings, err := deps.GetUserSettings.Handle(c.Context(), user.UserID)
		if err != nil {
			return err
		}

		// Resolved language code for <html lang=...>.
		ctx := c.Context()
		resolvedLang := "en"
		if locale := ctxi18n.Locale(ctx); locale != nil {
			resolvedLang = locale.Code().String()
		}

		curTheme, err := deps.GetUserThemeByID.Handle(c.Context(), user.UserID, settings.ThemeID)
		if err != nil {
			curTheme, _ = deps.EnsureDefaultTheme.Handle(c.Context(), user.UserID)
		}

		return middleware.Render(c, page.Dashboard(page.DashboardInput{
			BaseInput: layout.BaseInput{
				Title:    user.FirstName + "'s Dash",
				Language: resolvedLang,
				Theme: layout.Theme{
					Primary:   curTheme.Primary,
					Secondary: curTheme.Secondary,
					Tertiary:  curTheme.Tertiary,
				},
			},
			User: page.UserInfo{
				Picture:     user.Picture,
				DisplayName: user.DisplayName,
				ProfileUrl:  user.ProfileUrl,
			},
		}))
	}).Name(DashboardRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/dashboard/greeting", func(c fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			settings, err := deps.GetUserSettings.Handle(c.Context(), user.UserID)
			if err != nil {
				return err
			}

			tzName := settings.Timezone
			if tzName == "" || tzName == "auto" {
				tzName = tzCookie(c)
			}
			loc, err := time.LoadLocation(tzName)
			if err != nil {
				loc = time.UTC
			}
			localTime := time.Now().In(loc)

			ctx := c.Context()
			hour := localTime.Hour()
			var greetKey string
			switch {
			case hour >= 5 && hour < 12:
				greetKey = "greeting.good_morning"
			case hour >= 12 && hour < 17:
				greetKey = "greeting.good_afternoon"
			case hour >= 17 && hour < 22:
				greetKey = "greeting.good_evening"
			default:
				greetKey = "greeting.good_night"
			}
			greeting := fmt.Sprintf("%s, %s!", i18n.T(ctx, greetKey), user.FirstName)

			resolvedLang := "en"
			if locale := ctxi18n.Locale(ctx); locale != nil {
				resolvedLang = locale.Code().String()
			}
			date := webi18n.FormatDate(localTime, resolvedLang)

			return middleware.Render(c, partials.DashboardGreeting(partials.DashboardGreetingInput{
				Date:     date,
				Greeting: greeting,
			}))
		}).Name(DashboardGreetingRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/dashboard/title/applications", func(c fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}
			return middleware.Render(c, partials.DashboardTitleApplications(partials.DashboardTitleApplicationsInput{
				EditMode: false,
				IsAdmin:  user.IsAdmin,
			}))
		}).Name(DashboardTitleApplicationsRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/dashboard/title/applications/edit", func(c fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}
			return middleware.Render(c, partials.DashboardTitleApplications(partials.DashboardTitleApplicationsInput{
				EditMode: true,
				IsAdmin:  user.IsAdmin,
			}))
		}).Name(DashboardTitleApplicationsEditRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/dashboard/title/bookmarks", func(c fiber.Ctx) error {
			if _, authorized := middleware.GetCurrentUser(c); !authorized {
				return redirectToLogin(c)
			}
			return middleware.Render(c, partials.DashboardTitleBookmarks(partials.DashboardTitleBookmarksInput{
				EditMode: false,
			}))
		}).Name(DashboardTitleBookmarksRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/dashboard/title/bookmarks/edit", func(c fiber.Ctx) error {
			if _, authorized := middleware.GetCurrentUser(c); !authorized {
				return redirectToLogin(c)
			}
			return middleware.Render(c, partials.DashboardTitleBookmarks(partials.DashboardTitleBookmarksInput{
				EditMode: true,
			}))
		}).Name(DashboardTitleBookmarksEditRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/dashboard/edit/:mode", func(c fiber.Ctx) error {
			mode := c.Params("mode")
			if mode != "on" && mode != "off" {
				return fiber.NewError(fiber.StatusBadRequest, "invalid mode")
			}

			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}
			return middleware.Render(c, partials.DashboardEdit(partials.DashboardEditInput{
				EditMode:    mode == "on",
				WithTrigger: c.Query("initial") != "true",
				IsAdmin:     user.IsAdmin,
			}))
		}).Name(DashboardEditRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/dashboard/modal/close", func(c fiber.Ctx) error {
			if _, authorized := middleware.GetCurrentUser(c); !authorized {
				return redirectToLogin(c)
			}
			return c.SendString("")
		}).Name(DashboardModalCloseRoute)
}
