package domainerrors

import (
	"errors"
	"fmt"
	"testing"
)

func TestEntity_String(t *testing.T) {
	tests := []struct {
		entity Entity
		want   string
	}{
		{EntityDashboard, "dashboard"},
		{EntityCategory, "category"},
		{EntityBookmark, "bookmark"},
		{EntityTheme, "theme"},
		{EntitySetting, "setting"},
		{EntityApplication, "application"},
		{EntitySession, "session"},
		{EntityUnknown, "entity"},
		{Entity(9999), "entity"}, // unknown value falls through to default
	}

	for _, tt := range tests {
		t.Run(tt.want, func(t *testing.T) {
			if got := tt.entity.String(); got != tt.want {
				t.Errorf("Entity(%d).String() = %q, want %q", tt.entity, got, tt.want)
			}
		})
	}
}

func TestNotFoundError(t *testing.T) {
	err := NotFound(EntityCategory)
	if err == nil {
		t.Fatal("NotFound must not return nil")
	}
	want := "category not found"
	if err.Error() != want {
		t.Errorf("Error() = %q, want %q", err.Error(), want)
	}

	var nfe *NotFoundError
	if !errors.As(err, &nfe) {
		t.Error("NotFound should return *NotFoundError")
	}
}

func TestForbiddenError_WithReason(t *testing.T) {
	err := Forbidden("not your resource")
	want := "forbidden: not your resource"
	if err.Error() != want {
		t.Errorf("Error() = %q, want %q", err.Error(), want)
	}
}

func TestForbiddenError_WithoutReason(t *testing.T) {
	err := Forbidden("")
	want := "forbidden"
	if err.Error() != want {
		t.Errorf("Error() = %q, want %q", err.Error(), want)
	}
}

func TestValidationError_NoViolations(t *testing.T) {
	err := Validation()
	want := "validation error"
	if err.Error() != want {
		t.Errorf("Error() = %q, want %q", err.Error(), want)
	}
}

func TestValidationError_WithFieldViolation(t *testing.T) {
	err := Validation(Violation{Field: "email", Message: "required"})
	want := "validation error; email: required"
	if err.Error() != want {
		t.Errorf("Error() = %q, want %q", err.Error(), want)
	}
}

func TestValidationError_WithoutField(t *testing.T) {
	err := Validation(Violation{Message: "something went wrong"})
	want := "validation error; something went wrong"
	if err.Error() != want {
		t.Errorf("Error() = %q, want %q", err.Error(), want)
	}
}

func TestValidationError_MultipleViolations(t *testing.T) {
	err := Validation(
		Violation{Field: "name", Message: "required"},
		Violation{Field: "url", Message: "invalid"},
		Violation{Message: "global issue"},
	)
	msg := err.Error()
	for _, sub := range []string{"name: required", "url: invalid", "global issue"} {
		if !contains(msg, sub) {
			t.Errorf("Error() missing %q in %q", sub, msg)
		}
	}
}

func TestInternalError_WithCause(t *testing.T) {
	cause := errors.New("db timeout")
	err := Internal("create bookmark", cause)

	want := "internal error: create bookmark: db timeout"
	if err.Error() != want {
		t.Errorf("Error() = %q, want %q", err.Error(), want)
	}
	if !errors.Is(err, cause) {
		t.Error("Unwrap() should expose the cause")
	}
}

func TestInternalError_WithoutCause(t *testing.T) {
	err := Internal("migrate", nil)
	want := "internal error: migrate"
	if err.Error() != want {
		t.Errorf("Error() = %q, want %q", err.Error(), want)
	}
	if err.Unwrap() != nil {
		t.Error("Unwrap() should return nil when cause is nil")
	}
}

func TestWrapRepo_Nil(t *testing.T) {
	if got := WrapRepo("op", nil); got != nil {
		t.Errorf("WrapRepo(nil) = %v, want nil", got)
	}
}

func TestWrapRepo_NotFoundPassthrough(t *testing.T) {
	nfe := NotFound(EntityBookmark)
	got := WrapRepo("get bookmark", nfe)

	var result *NotFoundError
	if !errors.As(got, &result) {
		t.Errorf("WrapRepo(NotFoundError) should return *NotFoundError, got %T", got)
	}
}

func TestWrapRepo_OtherErrorWrapped(t *testing.T) {
	cause := fmt.Errorf("connection refused")
	got := WrapRepo("list categories", cause)

	var ie *InternalError
	if !errors.As(got, &ie) {
		t.Errorf("WrapRepo(generic error) should return *InternalError, got %T", got)
	}
	if !errors.Is(got, cause) {
		t.Error("WrapRepo should preserve the cause via Unwrap")
	}
}

func contains(s, sub string) bool {
	return len(s) >= len(sub) && (s == sub || len(s) > 0 && containsStr(s, sub))
}

func containsStr(s, sub string) bool {
	for i := 0; i <= len(s)-len(sub); i++ {
		if s[i:i+len(sub)] == sub {
			return true
		}
	}
	return false
}
