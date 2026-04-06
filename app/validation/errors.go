package validation

import (
	"errors"
	"strings"

	"github.com/go-playground/validator/v10"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
)

// Describe converts a validator.ValidationErrors or any error into a concise, human-friendly description.
// For unknown error types, it returns err.Error().
func Describe(err error) string {
	if err == nil {
		return ""
	}
	var verrs validator.ValidationErrors
	if errors.As(err, &verrs) {
		parts := make([]string, 0, len(verrs))
		for _, fe := range verrs {
			field := fe.Field()
			// Use the tag as the rule name; include param where useful
			if fe.Param() != "" {
				parts = append(parts, field+" "+fe.Tag()+"="+fe.Param())
			} else {
				parts = append(parts, field+" "+fe.Tag())
			}
		}
		return strings.Join(parts, ", ")
	}
	return err.Error()
}

// ToViolations converts a validator.ValidationErrors or any error into a []domainerrors.Violation.
func ToViolations(err error) []domainerrors.Violation {
	if err == nil {
		return nil
	}
	var verrs validator.ValidationErrors
	if errors.As(err, &verrs) {
		violations := make([]domainerrors.Violation, 0, len(verrs))
		for _, fe := range verrs {
			msg := fe.Tag()
			if fe.Param() != "" {
				msg = fe.Tag() + "=" + fe.Param()
			}
			violations = append(violations, domainerrors.Violation{
				Field:   fe.Field(),
				Message: msg,
			})
		}
		return violations
	}
	return []domainerrors.Violation{{Message: err.Error()}}
}
