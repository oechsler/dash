package command

import (
	"context"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainrepo "github.com/oechsler-it/dash/domain/repo"

	"github.com/oechsler-it/dash/app/validation"
)

// CreateUserThemeCmd is the input for creating a new theme.
type CreateUserThemeCmd struct {
	DisplayName string `validate:"required"`
	Primary     string `validate:"required,hexcolor"`
	Secondary   string `validate:"required,hexcolor"`
	Tertiary    string `validate:"required,hexcolor"`
}

// UserThemeCreator handles the CreateUserThemeCmd command.
type UserThemeCreator interface {
	Handle(ctx context.Context, userID string, in CreateUserThemeCmd) error
}

type CreateUserTheme struct {
	Repo      domainrepo.ThemeRepository
	Validator validation.Validator
}

func NewCreateUserTheme(r domainrepo.ThemeRepository, v validation.Validator) *CreateUserTheme {
	return &CreateUserTheme{
		Repo:      r,
		Validator: v,
	}
}

func (h *CreateUserTheme) Handle(ctx context.Context, userID string, in CreateUserThemeCmd) error {
	if err := h.Validator.Struct(in); err != nil {
		return domainerrors.Validation(validation.ToViolations(err)...)
	}

	record := &domainrepo.ThemeRecord{
		UserID:      userID,
		DisplayName: in.DisplayName,
		Primary:     in.Primary,
		Secondary:   in.Secondary,
		Tertiary:    in.Tertiary,
		Deletable:   true,
	}
	if err := h.Repo.Create(ctx, record); err != nil {
		return domainerrors.Internal("create user theme", err)
	}
	return nil
}
