package validation

import "github.com/go-playground/validator/v10"

// Validator abstracts the validator used in domain use-cases.
type Validator interface {
	Struct(any) error
}

// New returns a new validator instance implementing Validator.
func New() Validator {
	return validator.New()
}
