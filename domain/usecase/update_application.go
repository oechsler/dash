package usecase

import (
	"context"
	"dash/data/repo"
	"dash/domain/validation"
)

type UpdateApplicationInput struct {
	ID              uint     `validate:"required,gt=0"`
	Icon            string   `validate:"required"`
	DisplayName     string   `validate:"required"`
	Url             string   `validate:"required,url"`
	VisibleToGroups []string `validate:"dive,required"`
}

type UpdateApplication struct {
	ApplicationRepo repo.ApplicationRepo
	Validator       validation.Validator
}

func NewUpdateApplication(
	applicationRepo repo.ApplicationRepo,
	validator validation.Validator,
) *UpdateApplication {
	return &UpdateApplication{
		ApplicationRepo: applicationRepo,
		Validator:       validator,
	}
}

func (uc *UpdateApplication) Execute(ctx context.Context, in UpdateApplicationInput) error {
	if err := uc.Validator.Struct(in); err != nil {
		return Validation(err)
	}

	app, err := uc.ApplicationRepo.Get(ctx, in.ID)
	if err != nil {
		return Internal("update application: get", err)
	}
	if app == nil {
		return Forbidden("application not accessible", nil)
	}

	app.Icon = in.Icon
	app.DisplayName = in.DisplayName
	app.Url = in.Url
	app.VisibleToGroups = in.VisibleToGroups

	if err := uc.ApplicationRepo.Upsert(ctx, app); err != nil {
		return Internal("update application: upsert", err)
	}
	return nil
}
