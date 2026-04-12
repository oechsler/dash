package web

import (
	"github.com/gofiber/fiber/v3"
	"github.com/gofiber/fiber/v3/middleware/static"
)

func RegisterStaticFiles(app *fiber.App) {
	app.Get("/static*", static.New("./delivery/web/static"))
}
