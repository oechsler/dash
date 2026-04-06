package command

import (
	"context"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainmodel "github.com/oechsler-it/dash/domain/model"
	domainrepo "github.com/oechsler-it/dash/domain/repo"

	"github.com/oechsler-it/dash/app/validation"
)

// CreateApplicationCmd is the input for creating a new application link.
type CreateApplicationCmd struct {
	Icon            string   `validate:"required"`
	DisplayName     string   `validate:"required"`
	Url             string   `validate:"required,url"`
	VisibleToGroups []string `validate:"dive"`
}

// ApplicationCreator handles the CreateApplicationCmd command.
type ApplicationCreator interface {
	Handle(ctx context.Context, in CreateApplicationCmd) error
}

type CreateApplication struct {
	ApplicationRepo domainrepo.ApplicationRepository
	Validator       validation.Validator
}

func NewCreateApplication(
	applicationRepo domainrepo.ApplicationRepository,
	validator validation.Validator,
) *CreateApplication {
	return &CreateApplication{
		ApplicationRepo: applicationRepo,
		Validator:       validator,
	}
}

func (h *CreateApplication) Handle(ctx context.Context, in CreateApplicationCmd) error {
	if err := h.Validator.Struct(in); err != nil {
		return domainerrors.Validation(validation.ToViolations(err)...)
	}
	if _, err := domainmodel.ParseIcon(in.Icon); err != nil {
		return domainerrors.Validation(domainerrors.Violation{Message: err.Error()})
	}

	record := &domainrepo.ApplicationRecord{
		Icon:            in.Icon,
		DisplayName:     in.DisplayName,
		Url:             in.Url,
		VisibleToGroups: in.VisibleToGroups,
	}
	if err := h.ApplicationRepo.Upsert(ctx, record); err != nil {
		return domainerrors.Internal("create application: upsert", err)
	}
	return nil
}
