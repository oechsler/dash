package middleware

import (
	"github.com/a-h/templ"
	"github.com/gofiber/fiber/v3"
)

func Render(c fiber.Ctx, component templ.Component) error {
	c.Set("Content-Type", "text/html")
	return component.Render(c.Context(), c.Response().BodyWriter())
}
