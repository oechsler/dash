package middleware

import "github.com/gofiber/fiber/v2"

func HtmxOnly(c *fiber.Ctx) error {
	if c.Get("HX-Request") != "true" {
		return c.SendStatus(fiber.StatusForbidden)
	}
	return c.Next()
}
