package handler

import (
	"fmt"
	"io"
	"time"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	"git.at.oechsler.it/samuel/dash/v2/app/query"
	"git.at.oechsler.it/samuel/dash/v2/app/transfer"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/middleware"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/templ/partials"
	"git.at.oechsler.it/samuel/dash/v2/domain/model"
	"git.at.oechsler.it/samuel/dash/v2/infra/oidc"

	"github.com/gofiber/fiber/v3"
	"github.com/samber/lo"
)

const (
	SettingsModalRoute         = "SettingsModalRoute"
	SettingsModalThemesRoute   = "SettingsModalThemesRoute"
	SettingsUpdateRoute        = "SettingsUpdateRoute"
	SettingsExportRoute        = "SettingsExportRoute"
	SettingsImportRoute        = "SettingsImportRoute"
	SettingsDeleteAccountRoute = "SettingsDeleteAccountRoute"
	SettingsTokenRoute         = "SettingsTokenRoute"
)

var availableLanguages = []string{"auto", "en", "de"}

type timezone struct {
	IANA  string
	Label string
}

var availableTimezones = []timezone{
	{"auto", ""},
	{"UTC", "UTC (UTC±0)"},
	{"America/Los_Angeles", "America/Los_Angeles (UTC−8/−7)"},
	{"America/Denver", "America/Denver (UTC−7/−6)"},
	{"America/Chicago", "America/Chicago (UTC−6/−5)"},
	{"America/New_York", "America/New_York (UTC−5/−4)"},
	{"America/Sao_Paulo", "America/Sao_Paulo (UTC−3/−2)"},
	{"Atlantic/Azores", "Atlantic/Azores (UTC−1/0)"},
	{"Europe/London", "Europe/London (UTC+0/+1)"},
	{"Europe/Lisbon", "Europe/Lisbon (UTC+0/+1)"},
	{"Europe/Paris", "Europe/Paris (UTC+1/+2)"},
	{"Europe/Berlin", "Europe/Berlin (UTC+1/+2)"},
	{"Europe/Vienna", "Europe/Vienna (UTC+1/+2)"},
	{"Europe/Zurich", "Europe/Zurich (UTC+1/+2)"},
	{"Europe/Helsinki", "Europe/Helsinki (UTC+2/+3)"},
	{"Europe/Athens", "Europe/Athens (UTC+2/+3)"},
	{"Europe/Istanbul", "Europe/Istanbul (UTC+3)"},
	{"Europe/Moscow", "Europe/Moscow (UTC+3)"},
	{"Asia/Dubai", "Asia/Dubai (UTC+4)"},
	{"Asia/Kolkata", "Asia/Kolkata (UTC+5:30)"},
	{"Asia/Dhaka", "Asia/Dhaka (UTC+6)"},
	{"Asia/Bangkok", "Asia/Bangkok (UTC+7)"},
	{"Asia/Singapore", "Asia/Singapore (UTC+8)"},
	{"Asia/Shanghai", "Asia/Shanghai (UTC+8)"},
	{"Asia/Tokyo", "Asia/Tokyo (UTC+9)"},
	{"Australia/Adelaide", "Australia/Adelaide (UTC+9:30/+10:30)"},
	{"Australia/Sydney", "Australia/Sydney (UTC+10/+11)"},
	{"Pacific/Auckland", "Pacific/Auckland (UTC+12/+13)"},
}

type SettingDeps struct {
	SessionStore       *oidc.SessionStore
	App                *fiber.App
	GetUserSettings    query.UserSettingsGetter
	UpdateUserSettings command.UserSettingsUpdater
	ListUserThemes     query.UserThemesLister
	EnsureDefaultTheme command.DefaultThemeEnsurer
	ExportUserData     query.UserDataExporter
	DeleteUserData     command.UserDataDeleter
	ImportUserData     command.UserDataImporter
	BuildInfo          BuildInfo
}

// SettingPlain registers plain HTTP (non-HTMX) routes for settings.
// Must be called BEFORE any handler that invokes router.Use(HtmxOnly), because
// Fiber's Group.Use() registers middleware globally on the app, which would
// otherwise block all non-HTMX requests.
func SettingPlain(deps SettingDeps) {
	r := deps.App.
		Group("/").
		Use(middleware.LoadUserFromSession(deps.SessionStore))

	r.Get("/settings/export", func(c fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		export, err := deps.ExportUserData.Handle(c.Context(), user.UserID, user.Username, user.IsAdmin)
		if err != nil {
			return err
		}

		data, err := transfer.MarshalExport(export)
		if err != nil {
			return fiber.NewError(fiber.StatusInternalServerError, "failed to serialize export")
		}

		timestamp := time.Now().UTC().Format("20060102-150405")
		filename := fmt.Sprintf("dash-export-%s-%s.json", user.Username, timestamp)

		c.Set("Content-Type", "application/json")
		c.Set("Content-Disposition", fmt.Sprintf(`attachment; filename="%s"`, filename))
		return c.Send(data)
	}).Name(SettingsExportRoute)

	r.Get("/settings/token", func(c fiber.Ctx) error {
		_, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return fiber.NewError(fiber.StatusUnauthorized)
		}

		sessionData, ok := deps.SessionStore.Load(c)
		if !ok || sessionData.RawIDToken == "" {
			return fiber.NewError(fiber.StatusUnauthorized)
		}

		c.Set("Content-Type", "text/plain")
		return c.SendString(sessionData.RawIDToken)
	}).Name(SettingsTokenRoute)
}

func Setting(deps SettingDeps) {
	router := deps.App.
		Group("/").
		Use(middleware.LoadUserFromSession(deps.SessionStore))

	router.
		Use(middleware.HtmxOnly).
		Put("/settings", func(c fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			var body struct {
				ThemeID  uint   `form:"theme_id"`
				Language string `form:"language"`
				Timezone string `form:"timezone"`
			}
			if err := c.Bind().Body(&body); err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid body")
			}

			if err := deps.UpdateUserSettings.Handle(c.Context(), user.UserID, command.UpdateUserSettingsCmd{
				ThemeID:  body.ThemeID,
				Language: body.Language,
				Timezone: body.Timezone,
			}); err != nil {
				return err
			}

			c.Set("HX-Refresh", "true")
			return c.SendStatus(fiber.StatusNoContent)
		}).Name(SettingsUpdateRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/settings/modal", func(c fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			settings, err := deps.GetUserSettings.Handle(c.Context(), user.UserID)
			if err != nil {
				return err
			}

			themes, err := deps.ListUserThemes.Handle(c.Context(), user.UserID)
			if err != nil {
				return err
			}

			return middleware.Render(c, partials.SettingsModal(partials.SettingsModalInput{
				Settings: partials.SettingsModalInputSettings{
					ThemeID:  settings.ThemeID,
					Language: settings.Language,
					Timezone: settings.Timezone,
				},
				Themes: lo.Map(themes, func(theme model.Theme, _ int) partials.SettingsModalInputTheme {
					return partials.SettingsModalInputTheme{
						ID:          theme.ID,
						DisplayName: theme.Name,
					}
				}),
				Languages: lo.Map(availableLanguages, func(code string, _ int) partials.SettingsModalInputLanguage {
					return partials.SettingsModalInputLanguage{Code: code}
				}),
				Timezones: lo.Map(availableTimezones, func(tz timezone, _ int) partials.SettingsModalInputTimezone {
					return partials.SettingsModalInputTimezone{IANA: tz.IANA, Label: tz.Label}
				}),
				Build: partials.SettingsModalInputBuild{
					Version:   deps.BuildInfo.Version,
					Commit:    deps.BuildInfo.Commit,
					BuildDate: deps.BuildInfo.BuildDate,
					RepoURL:   deps.BuildInfo.RepoURL,
				},
			}))
		}).Name(SettingsModalRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/settings/modal/themes", func(c fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			settings, err := deps.GetUserSettings.Handle(c.Context(), user.UserID)
			if err != nil {
				return err
			}

			themes, err := deps.ListUserThemes.Handle(c.Context(), user.UserID)
			if err != nil {
				return err
			}

			currentTheme, _ := lo.Find(themes, func(t model.Theme) bool {
				return t.ID == settings.ThemeID
			})

			return middleware.Render(c, partials.SettingsModalThemeSection(partials.SettingsModalThemeSectionInput{
				Themes: lo.Map(themes, func(theme model.Theme, _ int) partials.SettingsModalThemeSectionInputTheme {
					return partials.SettingsModalThemeSectionInputTheme{
						ID:          theme.ID,
						DisplayName: theme.Name,
						Primary:     theme.Primary,
						Secondary:   theme.Secondary,
						Tertiary:    theme.Tertiary,
						Deletable:   theme.Deletable,
					}
				}),
				Current: partials.SettingsModalThemeSectionInputCurrentTheme{
					Primary:   currentTheme.Primary,
					Secondary: currentTheme.Secondary,
					Tertiary:  currentTheme.Tertiary,
				},
				Settings: &partials.SettingsModalThemeSectionInputSettings{
					ThemeID: settings.ThemeID,
				},
			}))
		}).Name(SettingsModalThemesRoute)

	// Import: HTMX, multipart file upload
	router.
		Use(middleware.HtmxOnly).
		Post("/settings/import", func(c fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			file, err := c.FormFile("file")
			if err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "missing file")
			}

			f, err := file.Open()
			if err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "cannot open file")
			}
			defer f.Close()

			raw, err := io.ReadAll(f)
			if err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "cannot read file")
			}

			export, err := transfer.UnmarshalExport(raw)
			if err != nil {
				return fiber.NewError(fiber.StatusBadRequest, err.Error())
			}

			if err := deps.ImportUserData.Handle(c.Context(), user.UserID, user.IsAdmin, export); err != nil {
				return err
			}

			c.Set("HX-Refresh", "true")
			return c.SendStatus(fiber.StatusNoContent)
		}).Name(SettingsImportRoute)

	// Delete account: HTMX, deletes all user data then triggers OIDC logout
	router.
		Use(middleware.HtmxOnly).
		Delete("/settings/account", func(c fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			if err := deps.DeleteUserData.Handle(c.Context(), user.UserID); err != nil {
				return err
			}

			// Redirect to the logout route which handles session clearing + OIDC end-session
			logoutURL, err := c.GetRouteURL(SessionLogoutRoute, fiber.Map{})
			if err != nil {
				logoutURL = "/session/logout"
			}
			c.Set("HX-Redirect", logoutURL)
			return c.SendStatus(fiber.StatusNoContent)
		}).Name(SettingsDeleteAccountRoute)

}
