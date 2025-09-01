package usecase

import (
	"context"
	"dash/data/repo"
	"dash/domain/validation"
	"fmt"
)

type UpdateApplicationInput struct {
	ID              uint     `validate:"required,gt=0"`
	Icon            string   `validate:"required"`
	DisplayName     string   `validate:"required"`
	Description     *string
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
		return fmt.Errorf("%w: %s", ErrValidation, validation.Describe(err))
	}

	app, err := uc.ApplicationRepo.Get(ctx, in.ID)
	if err != nil {
		return err
	}
	if app == nil {
		return ErrForbidden // treat as not found/forbidden as applications are admin-managed
	}

	app.Icon = in.Icon
	app.DisplayName = in.DisplayName
	app.Description = in.Description
	app.Url = in.Url
	app.VisibleToGroups = in.VisibleToGroups

	return uc.ApplicationRepo.Upsert(ctx, app)
}
