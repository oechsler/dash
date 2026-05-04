package validation

import "testing"

func TestNew(t *testing.T) {
	v := New()
	if v == nil {
		t.Fatal("New() must return a non-nil Validator")
	}

	// Verify it implements the Validator interface (compile-time check via assignment)
	var _ Validator = v

	// Verify it works: valid struct must not error
	type sample struct {
		Name string `validate:"required"`
	}
	if err := v.Struct(sample{Name: "ok"}); err != nil {
		t.Errorf("Struct on valid input returned error: %v", err)
	}

	// Invalid struct must return an error
	if err := v.Struct(sample{Name: ""}); err == nil {
		t.Error("Struct on invalid input should return an error")
	}
}
