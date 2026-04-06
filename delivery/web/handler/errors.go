package handler

import (
	"errors"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	"github.com/gofiber/fiber/v2"
)

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
