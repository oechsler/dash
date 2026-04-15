package domainerrors

import (
	"errors"
	"fmt"
)

// Entity identifies a domain entity in error messages.
type Entity int

const (
	EntityUnknown     Entity = iota
	EntityDashboard   Entity = iota
	EntityCategory    Entity = iota
	EntityBookmark    Entity = iota
	EntityTheme       Entity = iota
	EntitySetting     Entity = iota
	EntityApplication   Entity = iota
	EntityPinnedSession Entity = iota
)

func (e Entity) String() string {
	switch e {
	case EntityDashboard:
		return "dashboard"
	case EntityCategory:
		return "category"
	case EntityBookmark:
		return "bookmark"
	case EntityTheme:
		return "theme"
	case EntitySetting:
		return "setting"
	case EntityApplication:
		return "application"
	case EntityPinnedSession:
		return "pinned_session"
	default:
		return "entity"
	}
}

// NotFoundError is returned when an entity does not exist.
type NotFoundError struct {
	Entity Entity
}

func (e *NotFoundError) Error() string {
	return fmt.Sprintf("%s not found", e.Entity)
}

// ForbiddenError is returned when an action is not permitted.
type ForbiddenError struct {
	Reason string
}

func (e *ForbiddenError) Error() string {
	if e.Reason == "" {
		return "forbidden"
	}
	return fmt.Sprintf("forbidden: %s", e.Reason)
}

// Violation represents a single field-level validation failure.
type Violation struct {
	Field   string
	Message string
}

// ValidationError is returned when input does not satisfy domain or structural rules.
type ValidationError struct {
	Violations []Violation
}

func (e *ValidationError) Error() string {
	if len(e.Violations) == 0 {
		return "validation error"
	}
	msg := "validation error"
	for _, v := range e.Violations {
		if v.Field != "" {
			msg += fmt.Sprintf("; %s: %s", v.Field, v.Message)
		} else {
			msg += fmt.Sprintf("; %s", v.Message)
		}
	}
	return msg
}

// InternalError wraps infrastructure errors with operation context.
type InternalError struct {
	Op    string
	Cause error
}

func (e *InternalError) Error() string {
	if e.Cause == nil {
		return fmt.Sprintf("internal error: %s", e.Op)
	}
	return fmt.Sprintf("internal error: %s: %v", e.Op, e.Cause)
}

func (e *InternalError) Unwrap() error { return e.Cause }

// Constructors

func NotFound(entity Entity) *NotFoundError {
	return &NotFoundError{Entity: entity}
}

func Forbidden(reason string) *ForbiddenError {
	return &ForbiddenError{Reason: reason}
}

func Validation(violations ...Violation) *ValidationError {
	return &ValidationError{Violations: violations}
}

func Internal(op string, cause error) *InternalError {
	return &InternalError{Op: op, Cause: cause}
}

// WrapRepo propagates *NotFoundError unchanged;
// wraps any other error as *InternalError with the given operation label.
func WrapRepo(op string, err error) error {
	if err == nil {
		return nil
	}
	var nfe *NotFoundError
	if errors.As(err, &nfe) {
		return nfe
	}
	return Internal(op, err)
}
