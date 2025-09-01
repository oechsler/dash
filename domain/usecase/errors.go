package usecase

import (
	"dash/domain/validation"
	"errors"
	"fmt"
)

// Domain-level error helpers. Keep errors.Is/As friendly by exposing
// sentinel errors and returning wrapped errors that include context.
var (
	ErrForbidden               = errors.New("forbidden")
	ErrUserDoesNotOwnDashboard = errors.New("user does not own dashboard")
	ErrDashboardNotFound       = errors.New("dashboard not found")
	ErrCategoryNotFound        = errors.New("category not found")
	ErrBookmarkNotFound        = errors.New("bookmark not found")
	ErrThemeNotFound           = errors.New("theme not found")
	ErrSettingNotFound         = errors.New("setting not found")
	ErrValidation              = errors.New("validation error")
	ErrInternal                = errors.New("internal error")
)

func NotFound(entity string, cause error) error {
	if cause == nil {
		return fmt.Errorf("%s: %w", entity, errors.New("not found"))
	}
	return fmt.Errorf("%s: %w: %v", entity, errors.New("not found"), cause)
}

func Forbidden(msg string, cause error) error {
	if msg == "" {
		msg = "forbidden"
	}
	if cause == nil {
		return fmt.Errorf("%s: %w", msg, ErrForbidden)
	}
	return fmt.Errorf("%s: %w: %v", msg, ErrForbidden, cause)
}

func Internal(op string, cause error) error {
	if cause == nil {
		return fmt.Errorf("%s: %w", op, ErrInternal)
	}
	return fmt.Errorf("%s: %w: %v", op, ErrInternal, cause)
}

// Validation wraps validator or arbitrary errors into a domain validation error with a friendly description.
func Validation(err error) error {
	if err == nil {
		return nil
	}
	return fmt.Errorf("%w: %s", ErrValidation, validation.Describe(err))
}

// ValidationMsg is a convenience to create a validation error from a message.
func ValidationMsg(msg string) error {
	return fmt.Errorf("%w: %s", ErrValidation, msg)
}
