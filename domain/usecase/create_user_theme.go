package usecase

import (
	"context"
	datamodel "dash/data/model"
	"dash/data/repo"
	"dash/domain/validation"
)

type CreateUserThemeInput struct {
	DisplayName string `validate:"required"`
	Primary     string `validate:"required,hexcolor"`
	Secondary   string `validate:"required,hexcolor"`
	Tertiary    string `validate:"required,hexcolor"`
}

type CreateUserTheme struct {
	Repo      repo.ThemeRepo
	Validator validation.Validator
}

func NewCreateUserTheme(r repo.ThemeRepo, v validation.Validator) *CreateUserTheme {
	return &CreateUserTheme{
		Repo:      r,
		Validator: v,
	}
}

func (uc *CreateUserTheme) Execute(ctx context.Context, userID string, in CreateUserThemeInput) error {
	if err := uc.Validator.Struct(in); err != nil {
		return Validation(err)
	}

	t := &datamodel.Theme{
		UserId:      userID,
		DisplayName: in.DisplayName,
		Primary:     in.Primary,
		Secondary:   in.Secondary,
		Tertiary:    in.Tertiary,
		Deletable:   true,
	}
	if err := uc.Repo.Create(ctx, t); err != nil {
		return Internal("create user theme", err)
	}
	return nil
}
