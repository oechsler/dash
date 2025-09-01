package usecase

import "errors"

var (
	ErrForbidden               = errors.New("forbidden")
	ErrUserDoesNotOwnDashboard = errors.New("user does not own dashboard")
	ErrDashboardNotFound       = errors.New("dashboard not found")
	ErrCategoryNotFound        = errors.New("category not found")
	ErrBookmarkNotFound        = errors.New("bookmark not found")
	ErrValidation              = errors.New("validation error")
)
