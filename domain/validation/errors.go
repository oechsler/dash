package validation

import (
	"errors"
	"strings"

	"github.com/go-playground/validator/v10"
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
