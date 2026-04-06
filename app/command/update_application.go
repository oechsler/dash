package command

import (
	"context"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"

	"git.at.oechsler.it/samuel/dash/v2/app/validation"
)

// UpdateApplicationCmd is the input for updating an existing application link.
type UpdateApplicationCmd struct {
	ID              uint     `validate:"required,gt=0"`
	Icon            string   `validate:"required"`
	DisplayName     string   `validate:"required"`
	Url             string   `validate:"required,url"`
	VisibleToGroups []string `validate:"dive,required"`
}

// ApplicationUpdater handles the UpdateApplicationCmd command.
type ApplicationUpdater interface {
	Handle(ctx context.Context, in UpdateApplicationCmd) error
}

type UpdateApplication struct {
	ApplicationRepo domainrepo.ApplicationRepository
	Validator       validation.Validator
}

func NewUpdateApplication(
	applicationRepo domainrepo.ApplicationRepository,
	validator validation.Validator,
) *UpdateApplication {
	return &UpdateApplication{
		ApplicationRepo: applicationRepo,
		Validator:       validator,
	}
}

func (h *UpdateApplication) Handle(ctx context.Context, in UpdateApplicationCmd) error {
	if err := h.Validator.Struct(in); err != nil {
		return domainerrors.Validation(validation.ToViolations(err)...)
	}
	if _, err := domainmodel.ParseIcon(in.Icon); err != nil {
		return domainerrors.Validation(domainerrors.Violation{Message: err.Error()})
	}

	app, err := h.ApplicationRepo.Get(ctx, in.ID)
	if err != nil {
		return domainerrors.WrapRepo("update application: get", err)
	}

	app.Icon = in.Icon
	app.DisplayName = in.DisplayName
	app.Url = in.Url
	app.VisibleToGroups = in.VisibleToGroups

	if err := h.ApplicationRepo.Upsert(ctx, app); err != nil {
		return domainerrors.Internal("update application: upsert", err)
	}
	return nil
}
