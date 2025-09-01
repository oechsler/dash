package endpoint

import (
	"dash/domain/model"
	"dash/domain/usecase"
	"dash/middleware"
	"dash/templ/partials"

	"github.com/gofiber/fiber/v2"
	"github.com/samber/lo"
)

const (
	SettingsModalRoute       = "SettingsModalRoute"
	SettingsModalThemesRoute = "SettingsModalThemesRoute"
	SettingsUpdateRoute      = "SettingsUpdateRoute"
)

var availableLanguages = []fiber.Map{
	{"Code": "en", "DisplayName": "English"},
}

var availableTimeZones = []fiber.Map{
	{"TimeZone": "Local", "DisplayName": "Local"},
}

type SettingDeps struct {
	App                *fiber.App
	GetUserSettings    *usecase.GetUserSettings
	UpdateUserSettings *usecase.UpdateUserSettings
	ListUserThemes     *usecase.ListUserThemes
	EnsureDefaultTheme *usecase.EnsureDefaultTheme
}

func Setting(deps SettingDeps) {
	router := deps.App.
		Group("/").
		Use(middleware.GetUserFromIdToken)

	router.
		Use(middleware.HtmxOnly).
		Put("/settings", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			var body struct {
				ThemeID uint `form:"theme_id"`
			}
			if err := c.BodyParser(&body); err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid body")
			}

			if err := deps.UpdateUserSettings.Execute(c.Context(), user.ID, usecase.UpdateUserSettingsInput{
				ThemeID: body.ThemeID,
			}); err != nil {
				return err
			}

			c.Set("HX-Refresh", "true")
			return c.SendStatus(fiber.StatusNoContent)
		}).Name(SettingsUpdateRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/settings/modal", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			settings, err := deps.GetUserSettings.Execute(c.Context(), user.ID)
			if err != nil {
				return err
			}

			themes, err := deps.ListUserThemes.Execute(c.Context(), user.ID)
			if err != nil {
				return err
			}

			return middleware.Render(c, partials.SettingsModal(partials.SettingsModalInput{
				Settings: partials.SettingsModalInputSettings{
					ThemeID:  settings.ThemeID,
					Language: "en",
					TimeZone: "Local",
				},
				Themes: lo.Map(themes, func(theme model.Theme, _ int) partials.SettingsModalInputTheme {
					return partials.SettingsModalInputTheme{
						ID:          theme.ID,
						DisplayName: theme.Name,
					}
				}),
				Languages: lo.Map(availableLanguages, func(language fiber.Map, _ int) partials.SettingsModalInputLanguage {
					return partials.SettingsModalInputLanguage{
						Code:        language["Code"].(string),
						DisplayName: language["DisplayName"].(string),
					}
				}),
				TimeZones: lo.Map(availableTimeZones, func(timeZone fiber.Map, _ int) partials.SettingsModalInputTimeZone {
					return partials.SettingsModalInputTimeZone{
						TimeZone:    timeZone["TimeZone"].(string),
						DisplayName: timeZone["DisplayName"].(string),
					}
				}),
			}))
		}).Name(SettingsModalRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/settings/modal/themes", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			settings, err := deps.GetUserSettings.Execute(c.Context(), user.ID)
			if err != nil {
				return err
			}

			themes, err := deps.ListUserThemes.Execute(c.Context(), user.ID)
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
}
