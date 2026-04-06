package command

import (
	"context"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"

	"git.at.oechsler.it/samuel/dash/v2/app/validation"
)

// CreateUserCategoryCmd is the input for creating a new category.
type CreateUserCategoryCmd struct {
	DisplayName string `validate:"required"`
	IsShelved   bool
}

// UserCategoryCreator handles the CreateUserCategoryCmd command.
type UserCategoryCreator interface {
	Handle(ctx context.Context, userId string, in CreateUserCategoryCmd) error
}

type CreateUserCategory struct {
	DashboardRepo domainrepo.DashboardRepository
	CategoryRepo  domainrepo.CategoryRepository
	Validator     validation.Validator
}

func NewCreateUserCategory(
	dashboardRepo domainrepo.DashboardRepository,
	categoryRepo domainrepo.CategoryRepository,
	validator validation.Validator,
) *CreateUserCategory {
	return &CreateUserCategory{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
		Validator:     validator,
	}
}

func (h *CreateUserCategory) Handle(ctx context.Context, userId string, in CreateUserCategoryCmd) error {
	if err := h.Validator.Struct(in); err != nil {
		return domainerrors.Validation(validation.ToViolations(err)...)
	}

	dashRecord, err := h.DashboardRepo.GetByUserID(ctx, userId)
	if err != nil {
		return domainerrors.WrapRepo("create user category: get dashboard", err)
	}
	dash := domainmodel.NewUserDashboard(dashRecord.ID, dashRecord.UserID)

	cat, err := domainmodel.NewCategory(in.DisplayName)
	if err != nil {
		return domainerrors.Validation(domainerrors.Violation{Message: err.Error()})
	}
	if in.IsShelved {
		cat.Shelve()
	}

	if err := h.CategoryRepo.Upsert(ctx, &domainrepo.CategoryRecord{
		DashboardID: dash.ID(),
		DisplayName: cat.DisplayName,
		IsShelved:   cat.IsShelved,
	}); err != nil {
		return domainerrors.Internal("create user category: upsert", err)
	}
	return nil
}
