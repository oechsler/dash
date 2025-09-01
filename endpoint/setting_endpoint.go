package endpoint

import (
	"dash/domain/model"
	"dash/domain/usecase"
	"dash/middleware"

	"github.com/gofiber/fiber/v2"
)

// minimal language and timezone lists; can be expanded
var availableLanguages = []fiber.Map{
	{"Code": "en", "Name": "English"},
}

var availableTimeZones = []string{
	"UTC",
	"Local",
	"Europe/Berlin",
	"Europe/London",
	"Europe/Paris",
	"Europe/Madrid",
	"Europe/Rome",
	"Europe/Amsterdam",
	"Europe/Oslo",
	"Europe/Stockholm",
	"Europe/Copenhagen",
	"Europe/Helsinki",
	"Europe/Athens",
	"Europe/Dublin",
	"Europe/Warsaw",
	"Europe/Prague",
	"Europe/Vienna",
	"Europe/Lisbon",
	"Europe/Bucharest",
	"Europe/Sofia",
	"Europe/Kiev",
	"Europe/Zurich",
	"Africa/Cairo",
	"Africa/Johannesburg",
	"Africa/Lagos",
	"Africa/Nairobi",
	"Asia/Dubai",
	"Asia/Jerusalem",
	"Asia/Kolkata",
	"Asia/Kathmandu",
	"Asia/Dhaka",
	"Asia/Bangkok",
	"Asia/Singapore",
	"Asia/Hong_Kong",
	"Asia/Shanghai",
	"Asia/Tokyo",
	"Asia/Seoul",
	"Asia/Taipei",
	"Asia/Kuala_Lumpur",
	"Asia/Jakarta",
	"Australia/Sydney",
	"Australia/Melbourne",
	"Australia/Perth",
	"Australia/Brisbane",
	"Pacific/Auckland",
	"Pacific/Fiji",
	"Pacific/Honolulu",
	"America/Anchorage",
	"America/Los_Angeles",
	"America/Denver",
	"America/Chicago",
	"America/New_York",
	"America/Toronto",
	"America/Mexico_City",
	"America/Bogota",
	"America/Lima",
	"America/Santiago",
	"America/Argentina/Buenos_Aires",
	"America/Sao_Paulo",
	"America/Caracas",
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

	router.Get("/settings", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		settings, err := deps.GetUserSettings.Execute(c.Context(), user.ID)
		if err != nil {
			return err
		}
		// ensure default theme exists and list themes
		cur, err := deps.EnsureDefaultTheme.Execute(c.Context(), user.ID)
		if err != nil {
			return err
		}
		themes, err := deps.ListUserThemes.Execute(c.Context(), user.ID)
		if err != nil {
			return err
		}
		return c.Render("settings", fiber.Map{
			"settings":  settings,
			"themes":    themes,
			"theme":     fiber.Map{"Primary": cur.Primary, "Secondary": cur.Secondary, "Tertiary": cur.Tertiary},
			"languages": availableLanguages,
			"timezones": availableTimeZones,
			"user": fiber.Map{
				"display_name": user.DisplayName,
				"username":     user.Username,
				"picture":      user.Picture,
			},
		})
	})

	router.Get("/settings/modal", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}
		settings, err := deps.GetUserSettings.Execute(c.Context(), user.ID)
		if err != nil {
			return err
		}
		cur, err := deps.EnsureDefaultTheme.Execute(c.Context(), user.ID)
		if err != nil {
			return err
		}
		// themes section list is lazy-loaded; keep themes here for the Theme select options
		themes, err := deps.ListUserThemes.Execute(c.Context(), user.ID)
		if err != nil {
			return err
		}
		return c.Render("partials/modal-settings", fiber.Map{
			"settings":  settings,
			"themes":    themes,
			"theme":     fiber.Map{"Primary": cur.Primary, "Secondary": cur.Secondary, "Tertiary": cur.Tertiary},
			"languages": availableLanguages,
			"timezones": availableTimeZones,
		})
	})

	// Serves the themes section partial for HTMX lazy load inside settings modal
	router.Get("/settings/modal/themes", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}
		settings, err := deps.GetUserSettings.Execute(c.Context(), user.ID)
		if err != nil {
			return err
		}
		list, err := deps.ListUserThemes.Execute(c.Context(), user.ID)
		if err != nil {
			return err
		}
		return c.Render("partials/settings-themes-section", fiber.Map{"themes": list, "settings": settings})
	})

	router.Post("/settings", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}
		var input model.Setting
		if err := c.BodyParser(&input); err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid body")
		}
		if _, err := deps.UpdateUserSettings.Execute(c.Context(), user.ID, input); err != nil {
			return err
		}
		// On HTMX request, trigger a full page reload; otherwise redirect to /settings
		if c.Get("HX-Request") == "true" {
			c.Set("HX-Refresh", "true")
			return c.SendStatus(fiber.StatusNoContent)
		}
		return c.Redirect("/settings")
	})
}
