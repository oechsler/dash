package endpoint

import (
	"dash/domain/usecase"
	"dash/middleware"
	"dash/templ/layout"
	"dash/templ/page"
	"dash/templ/partials"
	"time"

	"github.com/gofiber/fiber/v2"
)

const (
	DashboardRoute                      = "DashboardRoute"
	DashboardTitleApplicationsRoute     = "DashboardTitleApplicationsRoute"
	DashboardTitleApplicationsEditRoute = "DashboardTitleApplicationsEditRoute"
	DashboardTitleBookmarksRoute        = "DashboardTitleBookmarksRoute"
	DashboardTitleBookmarksEditRoute    = "DashboardTitleBookmarksEditRoute"
	DashboardEditRoute                  = "DashboardEditRoute"
	DashboardModalCloseRoute            = "DashboardModalCloseRoute"
)

type DashboardDeps struct {
	App                *fiber.App
	GetUserDashboard   *usecase.GetUserDashboard
	GetUserSettings    *usecase.GetUserSettings
	EnsureDefaultTheme *usecase.EnsureDefaultTheme
	GetUserThemeByID   *usecase.GetUserThemeByID
}

func Dashboard(deps DashboardDeps) {
	router := deps.App.
		Group("/").
		Use(middleware.GetUserFromIdToken)

	router.Get("/", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		dashboard, err := deps.GetUserDashboard.Execute(
			c.Context(),
			user.ID,
			user.Groups,
			user.FirstName,
			time.Time{},
		)
		if err != nil {
			return err
		}

		settings, err := deps.GetUserSettings.Execute(c.Context(), user.ID)
		if err != nil {
			return err
		}

		curTheme, err := deps.GetUserThemeByID.Execute(c.Context(), user.ID, settings.ThemeID)
		if err != nil {
			curTheme, _ = deps.EnsureDefaultTheme.Execute(c.Context(), user.ID)
		}

		return middleware.Render(c, page.Dashboard(page.DashboardInput{
			BaseInput: layout.BaseInput{
				Title:    user.FirstName + "'s Dash",
				Language: "en",
				Theme: layout.Theme{
					Primary:   curTheme.Primary,
					Secondary: curTheme.Secondary,
					Tertiary:  curTheme.Tertiary,
				},
			},
			User: page.UserInfo{
				Picture:     user.Picture,
				DisplayName: user.DisplayName,
			},
			Date:     dashboard.Greeting.Date,
			Greeting: dashboard.Greeting.Message,
		}))
	}).Name(DashboardRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/dashboard/title/applications", func(c *fiber.Ctx) error {
			user, _ := middleware.GetCurrentUser(c)
			return middleware.Render(c, partials.DashboardTitleApplications(partials.DashboardTitleApplicationsInput{
				EditMode: false,
				IsAdmin:  user.IsAdmin,
			}))
		}).Name(DashboardTitleApplicationsRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/dashboard/title/applications/edit", func(c *fiber.Ctx) error {
			user, _ := middleware.GetCurrentUser(c)
			return middleware.Render(c, partials.DashboardTitleApplications(partials.DashboardTitleApplicationsInput{
				EditMode: true,
				IsAdmin:  user.IsAdmin,
			}))
		}).Name(DashboardTitleApplicationsEditRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/dashboard/title/bookmarks", func(c *fiber.Ctx) error {
			return middleware.Render(c, partials.DashboardTitleBookmarks(partials.DashboardTitleBookmarksInput{
				EditMode: false,
			}))
		}).Name(DashboardTitleBookmarksRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/dashboard/title/bookmarks/edit", func(c *fiber.Ctx) error {
			return middleware.Render(c, partials.DashboardTitleBookmarks(partials.DashboardTitleBookmarksInput{
				EditMode: true,
			}))
		}).Name(DashboardTitleBookmarksEditRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/dashboard/edit/:mode", func(c *fiber.Ctx) error {
			mode := c.Params("mode")
			if mode != "on" && mode != "off" {
				return fiber.NewError(fiber.StatusBadRequest, "invalid mode")
			}

			user, _ := middleware.GetCurrentUser(c)
			return middleware.Render(c, partials.DashboardEdit(partials.DashboardEditInput{
				EditMode:    mode == "on",
				WithTrigger: true,
				IsAdmin:     user.IsAdmin,
			}))
		}).Name(DashboardEditRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/dashboard/modal/close", func(c *fiber.Ctx) error {
			return c.SendString("")
		}).Name(DashboardModalCloseRoute)
}
