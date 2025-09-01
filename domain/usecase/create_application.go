package usecase

import (
	"context"
	"dash/data/model"
	"dash/data/repo"
	"dash/domain/validation"
	"fmt"
)

type CreateApplicationInput struct {
	Icon            string `validate:"required"`
	DisplayName     string `validate:"required"`
	Description     *string
	Url             string   `validate:"required,url"`
	VisibleToGroups []string `validate:"dive"`
}

type CreateApplication struct {
	ApplicationRepo repo.ApplicationRepo
	Validator       validation.Validator
}

func NewCreateApplication(
	applicationRepo repo.ApplicationRepo,
	validator validation.Validator,
) *CreateApplication {
	return &CreateApplication{
		ApplicationRepo: applicationRepo,
		Validator:       validator,
	}
}

func (uc *CreateApplication) Execute(ctx context.Context, in CreateApplicationInput) error {
	if err := uc.Validator.Struct(in); err != nil {
		return fmt.Errorf("%w: %s", ErrValidation, validation.Describe(err))
	}

	app := &model.Application{
		Icon:            in.Icon,
		DisplayName:     in.DisplayName,
		Description:     in.Description,
		Url:             in.Url,
		VisibleToGroups: in.VisibleToGroups,
	}
	return uc.ApplicationRepo.Upsert(ctx, app)
}
