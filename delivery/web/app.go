package web

import (
	"log"
	"time"

	"github.com/oechsler-it/dash/config"

	"github.com/gofiber/fiber/v2"
)

func NewFiberApp(cfg *config.AppConfig) *fiber.App {
	app := fiber.New(fiber.Config{
		DisableStartupMessage: true,
		ReadBufferSize:        1024 * 1024 * 1,
	})

	app.Use(func(c *fiber.Ctx) error {
		start := time.Now()
		err := c.Next()

		log.Printf("[%d] %s %s - %v - %s - %s",
			c.Response().StatusCode(),
			c.Method(),
			c.Path(),
			time.Since(start),
			c.IP(),
			c.Get("User-Agent"),
		)

		return err
	})

	return app
}
