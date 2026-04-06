package handler

import "github.com/gofiber/fiber/v2"

func Health(app *fiber.App) {
	app.Get("/health", func(c *fiber.Ctx) error {
		return c.SendStatus(fiber.StatusOK)
	})
}
