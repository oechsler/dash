package endpoint

import (
	"dash/domain/usecase"
	"dash/middleware"
	"time"

	"github.com/gofiber/fiber/v2"
)

const (
	DashboardRoute = "DashboardRoute"
)

type DashboardDeps struct {
	App                 *fiber.App
	GetUserDashboard    *usecase.GetUserDashboard
	GetUserSettings     *usecase.GetUserSettings
	EnsureDefaultTheme  *usecase.EnsureDefaultTheme
	GetUserThemeByID    *usecase.GetUserThemeByID
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

		// theme
		settings, err := deps.GetUserSettings.Execute(c.Context(), user.ID)
		if err != nil { return err }
		curTheme, err := deps.GetUserThemeByID.Execute(c.Context(), user.ID, settings.ThemeID)
		if err != nil { curTheme, _ = deps.EnsureDefaultTheme.Execute(c.Context(), user.ID) }
		return c.Render("dashboard", fiber.Map{
			"date":     dashboard.Greeting.Date,
			"greeting": dashboard.Greeting.Message,
			"user": fiber.Map{
				"display_name": user.DisplayName,
				"username":     user.Username,
				"picture":      user.Picture,
			},
			"theme": fiber.Map{"Primary": curTheme.Primary, "Secondary": curTheme.Secondary, "Tertiary": curTheme.Tertiary},
			"dashboard": dashboard,
		})
	}).Name(DashboardRoute)

	router.Get("/dashboard/title/applications", func(c *fiber.Ctx) error {
		user, _ := middleware.GetCurrentUser(c)
		return c.Render("partials/dashboard-title-applications", fiber.Map{
			"edit_mode": false,
			"is_admin":  user.IsAdmin,
		})
	})

	router.Get("/dashboard/title/applications/edit", func(c *fiber.Ctx) error {
		user, _ := middleware.GetCurrentUser(c)
		return c.Render("partials/dashboard-title-applications", fiber.Map{
			"edit_mode": true,
			"is_admin":  user.IsAdmin,
		})
	})

	router.Get("/dashboard/title/bookmarks", func(c *fiber.Ctx) error {
		return c.Render("partials/dashboard-title-bookmarks", fiber.Map{
			"edit_mode": false,
		})
	})

	router.Get("/dashboard/title/bookmarks/edit", func(c *fiber.Ctx) error {
		return c.Render("partials/dashboard-title-bookmarks", fiber.Map{
			"edit_mode": true,
		})
	})

	router.Get("/dashboard/title/shelved", func(c *fiber.Ctx) error {
		return c.Render("partials/dashboard-title-shelved", fiber.Map{
			"edit_mode": false,
		})
	})

	router.Get("/dashboard/title/shelved/edit", func(c *fiber.Ctx) error {
		return c.Render("partials/dashboard-title-shelved", fiber.Map{
			"edit_mode": true,
		})
	})

	router.Get("/dashboard/edit", func(c *fiber.Ctx) error {
		user, _ := middleware.GetCurrentUser(c)
		return c.Render("partials/dashboard-edit", fiber.Map{
			"edit_mode":    false,
			"with_trigger": false,
			"is_admin":     user.IsAdmin,
		})
	})

	router.Get("/dashboard/edit/on", func(c *fiber.Ctx) error {
		user, _ := middleware.GetCurrentUser(c)
		return c.Render("partials/dashboard-edit", fiber.Map{
			"edit_mode":    true,
			"with_trigger": true,
			"is_admin":     user.IsAdmin,
		})
	})

	router.Get("/dashboard/edit/off", func(c *fiber.Ctx) error {
		user, _ := middleware.GetCurrentUser(c)
		return c.Render("partials/dashboard-edit", fiber.Map{
			"edit_mode":    false,
			"with_trigger": true,
			"is_admin":     user.IsAdmin,
		})
	})

	// moved to /applications/modal/create

	// moved to /applications/modal/delete/:id

	/*	router.Get("/dashboard/modal/edit", func(c *fiber.Ctx) error {
			return c.Render("partials/modal-edit", nil)
		})

		router.Get("/dashboard/modal/create", func(c *fiber.Ctx) error {
			return c.Render("partials/modal-create", nil)
		})

		// Delete confirmation modal. Expects query params:
		//  - name: optional display name to show in confirmation
		//  - path: required path to send DELETE request to (e.g., /categories/1)
		//  - target: required CSS selector for htmx target to swap (e.g., #categories-list)
		//  - swap: required htmx swap mode (e.g., innerHTML)
		//  - mode: optional value for X-Mode header (e.g., edit)
		router.Get("/dashboard/modal/delete", func(c *fiber.Ctx) error {
			target := c.Query("target")
			// Normalize CSS selector for hx-target so that callers can pass %23id, #id, or id
			if target != "" {
				if strings.HasPrefix(target, "%23") {
					target = "#" + target[3:]
				} else if !strings.HasPrefix(target, "#") {
					target = "#" + target
				}
			}
			data := fiber.Map{
				"name":   c.Query("name"),
				"path":   c.Query("path"),
				"target": target,
				"swap":   c.Query("swap", "innerHTML"),
				"mode":   c.Query("mode"),
			}
			return c.Render("partials/modal-delete", fiber.Map{"data": data})
		})*/

	router.Get("/dashboard/modal/close", func(c *fiber.Ctx) error {
		return c.SendString("")
	})
}
