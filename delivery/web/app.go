package web

import (
	"log"
	"time"

	"git.at.oechsler.it/samuel/dash/v2/config"

	"github.com/gofiber/fiber/v3"
)

func NewFiberApp(cfg *config.AppConfig) *fiber.App {
	app := fiber.New(fiber.Config{
		ReadBufferSize: 1024 * 1024 * 1,
		TrustProxy:     true,
		TrustProxyConfig: fiber.TrustProxyConfig{
			Private: true,
		},
	})

	app.Use(func(c fiber.Ctx) error {
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
