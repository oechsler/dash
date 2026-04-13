package web

import (
	"embed"
	"io/fs"

	"github.com/gofiber/fiber/v3"
	"github.com/gofiber/fiber/v3/middleware/static"
)

//go:embed static
var staticFiles embed.FS

func RegisterStaticFiles(app *fiber.App) {
	sub, err := fs.Sub(staticFiles, "static")
	if err != nil {
		panic("static: failed to create sub-filesystem: " + err.Error())
	}
	app.Get("/static*", static.New("", static.Config{FS: sub}))
}
