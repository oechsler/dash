package handler

import (
	"errors"
	"net/url"
	"time"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	"github.com/gofiber/fiber/v3"
)

// tzCookie reads the "tz" cookie and URL-decodes it (browsers may have stored
// an encoded value like "Europe%2FBerlin" from a previous version).
func tzCookie(c fiber.Ctx) string {
	raw := c.Cookies("tz", "")
	if raw == "" {
		return "UTC"
	}
	if decoded, err := url.QueryUnescape(raw); err == nil {
		if _, err := time.LoadLocation(decoded); err == nil {
			return decoded
		}
	}
	if _, err := time.LoadLocation(raw); err == nil {
		return raw
	}
	return "UTC"
}

// httpError maps a domain error to a Fiber HTTP error.
// Unrecognized errors (e.g. *InternalError) are returned as-is so Fiber's
// default error handler treats them as 500 Internal Server Error.
func httpError(err error) error {
	if err == nil {
		return nil
	}
	var nfe *domainerrors.NotFoundError
	if errors.As(err, &nfe) {
		return fiber.NewError(fiber.StatusNotFound, nfe.Error())
	}
	var fe *domainerrors.ForbiddenError
	if errors.As(err, &fe) {
		return fiber.NewError(fiber.StatusForbidden, fe.Error())
	}
	var ve *domainerrors.ValidationError
	if errors.As(err, &ve) {
		return fiber.NewError(fiber.StatusBadRequest, ve.Error())
	}
	return err
}
