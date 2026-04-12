package handler

import (
	"strconv"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	"git.at.oechsler.it/samuel/dash/v2/app/query"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/middleware"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/templ/partials"
	"git.at.oechsler.it/samuel/dash/v2/domain/model"
	"git.at.oechsler.it/samuel/dash/v2/infra/oidc"

	"github.com/gofiber/fiber/v3"
	"github.com/samber/lo"
)

const (
	ThemeCreateRoute = "ThemeCreateRoute"
	ThemeDeleteRoute = "ThemeDeleteRoute"
)

type ThemeDeps struct {
	SessionStore    *oidc.SessionStore
	App             *fiber.App
	ListUserThemes  query.UserThemesLister
	CreateUserTheme command.UserThemeCreator
	DeleteUserTheme command.UserThemeDeleter
	GetUserSettings query.UserSettingsGetter
}

func Theme(deps ThemeDeps) {
	router := deps.App.
		Group("/").
		Use(middleware.LoadUserFromSession(deps.SessionStore))

	router.
		Use(middleware.HtmxOnly).
		Post("/themes", func(c fiber.Ctx) error {
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
			if err := c.Bind().Body(&body); err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid body")
			}

			if err := deps.CreateUserTheme.Handle(
				c.Context(),
				user.UserID,
				command.CreateUserThemeCmd{
					DisplayName: body.DisplayName,
					Primary:     body.Primary,
					Secondary:   body.Secondary,
					Tertiary:    body.Tertiary,
				},
			); err != nil {
				return err
			}

			themes, err := deps.ListUserThemes.Handle(c.Context(), user.UserID)
			if err != nil {
				return err
			}

			settings, err := deps.GetUserSettings.Handle(c.Context(), user.UserID)
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
		Delete("/themes/:id", func(c fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			id64, err := strconv.ParseUint(c.Params("id"), 10, 64)
			if err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid id")
			}

			if err := deps.DeleteUserTheme.Handle(c.Context(), user.UserID, uint(id64)); err != nil {
				return err
			}

			themes, err := deps.ListUserThemes.Handle(c.Context(), user.UserID)
			if err != nil {
				return err
			}

			settings, err := deps.GetUserSettings.Handle(c.Context(), user.UserID)
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
