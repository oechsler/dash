package validation

import (
	"errors"
	"testing"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	"github.com/go-playground/validator/v10"
)

// validationErrorsFor runs validation on a struct and returns the raw error.
func validationErrorsFor(t *testing.T, s any) error {
	t.Helper()
	v := validator.New()
	return v.Struct(s)
}

// Structs for triggering specific validation errors.
type requiredOnly struct {
	Name string `validate:"required"`
}

type minLength struct {
	Name string `validate:"required,min=3"`
}

func TestDescribe_Nil(t *testing.T) {
	if got := Describe(nil); got != "" {
		t.Errorf("Describe(nil) = %q, want \"\"", got)
	}
}

func TestDescribe_ValidationErrors(t *testing.T) {
	err := validationErrorsFor(t, requiredOnly{Name: ""})
	if err == nil {
		t.Fatal("expected validation error")
	}
	got := Describe(err)
	if got == "" {
		t.Error("Describe(ValidationErrors) should return non-empty string")
	}
	// Should mention the field name
	if !containsSub(got, "Name") {
		t.Errorf("Describe result %q should contain field name \"Name\"", got)
	}
}

func TestDescribe_ValidationErrorsWithParam(t *testing.T) {
	err := validationErrorsFor(t, minLength{Name: "ab"})
	if err == nil {
		t.Fatal("expected validation error")
	}
	got := Describe(err)
	// Should mention the param (min=3)
	if !containsSub(got, "3") {
		t.Errorf("Describe result %q should contain param \"3\"", got)
	}
}

func TestDescribe_NonValidationError(t *testing.T) {
	err := errors.New("something broke")
	got := Describe(err)
	if got != "something broke" {
		t.Errorf("Describe(generic error) = %q, want \"something broke\"", got)
	}
}

func TestToViolations_Nil(t *testing.T) {
	if got := ToViolations(nil); got != nil {
		t.Errorf("ToViolations(nil) = %v, want nil", got)
	}
}

func TestToViolations_ValidationErrors(t *testing.T) {
	err := validationErrorsFor(t, requiredOnly{Name: ""})
	if err == nil {
		t.Fatal("expected validation error")
	}
	violations := ToViolations(err)
	if len(violations) == 0 {
		t.Error("ToViolations should return at least one violation")
	}
	if violations[0].Field == "" {
		t.Error("violation should have a non-empty field name")
	}
	if violations[0].Message == "" {
		t.Error("violation should have a non-empty message")
	}
}

func TestToViolations_ValidationErrorsWithParam(t *testing.T) {
	err := validationErrorsFor(t, minLength{Name: "ab"})
	if err == nil {
		t.Fatal("expected validation error")
	}
	violations := ToViolations(err)
	if len(violations) == 0 {
		t.Fatal("expected at least one violation")
	}
	// Message should contain the param
	if !containsSub(violations[0].Message, "3") {
		t.Errorf("violation message %q should contain param \"3\"", violations[0].Message)
	}
}

func TestToViolations_NonValidationError(t *testing.T) {
	err := errors.New("generic error")
	violations := ToViolations(err)
	if len(violations) != 1 {
		t.Fatalf("expected 1 violation for generic error, got %d", len(violations))
	}
	if violations[0].Field != "" {
		t.Errorf("generic error violation should have empty field, got %q", violations[0].Field)
	}
	if violations[0].Message != "generic error" {
		t.Errorf("violation message = %q, want \"generic error\"", violations[0].Message)
	}
}

func TestToViolations_ReturnsDomainViolations(t *testing.T) {
	err := validationErrorsFor(t, requiredOnly{})
	violations := ToViolations(err)
	for _, v := range violations {
		// Ensure they are properly typed as domainerrors.Violation
		_ = domainerrors.Violation(v)
	}
}

func containsSub(s, sub string) bool {
	for i := 0; i <= len(s)-len(sub); i++ {
		if s[i:i+len(sub)] == sub {
			return true
		}
	}
	return false
}
