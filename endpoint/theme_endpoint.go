package endpoint

import (
	"dash/domain/model"
	"dash/domain/usecase"
	"dash/environment"
	"dash/middleware"
	"dash/templ/partials"
	"strconv"

	"github.com/gofiber/fiber/v2"
	"github.com/samber/lo"
)

const (
	ThemeCreateRoute = "ThemeCreateRoute"
	ThemeDeleteRoute = "ThemeDeleteRoute"
)

type ThemeDeps struct {
	Env             *environment.Env
	App             *fiber.App
	ListUserThemes  *usecase.ListUserThemes
	CreateUserTheme *usecase.CreateUserTheme
	DeleteUserTheme *usecase.DeleteUserTheme
	GetUserSettings *usecase.GetUserSettings
}

func Theme(deps ThemeDeps) {
	router := deps.App.
		Group("/").
		Use(middleware.GetUserFromIdToken(deps.Env))

	router.
		Use(middleware.HtmxOnly).
		Post("/themes", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			var body struct {
				DisplayName string `form:"display_name"`
				Primary     string `form:"primary"`
				Secondary   string `form:"secondary"`
				Tertiary    string `form:"tertiary"`
			}
			if err := c.BodyParser(&body); err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid body")
			}

			if err := deps.CreateUserTheme.Execute(
				c.Context(),
				user.ID,
				usecase.CreateUserThemeInput{
					DisplayName: body.DisplayName,
					Primary:     body.Primary,
					Secondary:   body.Secondary,
					Tertiary:    body.Tertiary,
				},
			); err != nil {
				return err
			}

			themes, err := deps.ListUserThemes.Execute(c.Context(), user.ID)
			if err != nil {
				return err
			}

			settings, err := deps.GetUserSettings.Execute(c.Context(), user.ID)
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
		}).Name(ThemeCreateRoute)

	router.
		Use(middleware.HtmxOnly).
		Delete("/themes/:id", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			id64, err := strconv.ParseUint(c.Params("id"), 10, 64)
			if err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid id")
			}

			if err := deps.DeleteUserTheme.Execute(c.Context(), user.ID, uint(id64)); err != nil {
				return err
			}

			themes, err := deps.ListUserThemes.Execute(c.Context(), user.ID)
			if err != nil {
				return err
			}

			settings, err := deps.GetUserSettings.Execute(c.Context(), user.ID)
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
		}).Name(ThemeDeleteRoute)
}
