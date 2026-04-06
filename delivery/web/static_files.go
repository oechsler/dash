package web

import "github.com/gofiber/fiber/v2"

func RegisterStaticFiles(app *fiber.App) {
	app.Static("/static", "./delivery/web/static")
}
