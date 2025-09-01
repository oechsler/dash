package endpoint

import (
	"dash/domain/usecase"
	"dash/middleware"
	"strconv"

	"github.com/gofiber/fiber/v2"
)

type ThemeDeps struct {
	App             *fiber.App
	ListUserThemes  *usecase.ListUserThemes
	CreateUserTheme *usecase.CreateUserTheme
	DeleteUserTheme *usecase.DeleteUserTheme
	GetUserSettings *usecase.GetUserSettings
}

func Theme(deps ThemeDeps) {
	router := deps.App.
		Group("/").
		Use(middleware.GetUserFromIdToken)

	router.Post("/themes", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}
		name := c.FormValue("name")
		primary := c.FormValue("primary")
		secondary := c.FormValue("secondary")
		tertiary := c.FormValue("tertiary")
		if _, err := deps.CreateUserTheme.Execute(c.Context(), user.ID, name, primary, secondary, tertiary); err != nil {
			return err
		}
		// If request comes from HTMX, re-render themes section only (keep modal open)
		if c.Get("HX-Request") == "true" {
			s, err := deps.GetUserSettings.Execute(c.Context(), user.ID)
			if err != nil { return err }
			list, err := deps.ListUserThemes.Execute(c.Context(), user.ID)
			if err != nil { return err }
			return c.Render("partials/settings-themes-section", fiber.Map{"themes": list, "settings": s})
		}
		return c.Redirect("/settings")
	})

	router.Post("/themes/:id/delete", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}
		n, err := strconv.ParseUint(c.Params("id"), 10, 64)
		if err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid id")
		}
		if err := deps.DeleteUserTheme.Execute(c.Context(), user.ID, uint(n)); err != nil {
			return err
		}
		if c.Get("HX-Request") == "true" {
			s, err := deps.GetUserSettings.Execute(c.Context(), user.ID)
			if err != nil { return err }
			list, err := deps.ListUserThemes.Execute(c.Context(), user.ID)
			if err != nil { return err }
			return c.Render("partials/settings-themes-section", fiber.Map{"themes": list, "settings": s})
		}
		return c.Redirect("/settings")
	})
}
