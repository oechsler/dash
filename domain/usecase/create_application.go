package usecase

import (
	"context"
	"dash/data/model"
	"dash/data/repo"
	"dash/domain/validation"
)

type CreateApplicationInput struct {
	Icon            string   `validate:"required"`
	DisplayName     string   `validate:"required"`
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
		return Validation(err)
	}

	app := &model.Application{
		Icon:            in.Icon,
		DisplayName:     in.DisplayName,
		Url:             in.Url,
		VisibleToGroups: in.VisibleToGroups,
	}
	if err := uc.ApplicationRepo.Upsert(ctx, app); err != nil {
		return Internal("create application: upsert", err)
	}
	return nil
}
