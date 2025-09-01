package usecase

import (
	"context"
	"dash/data/model"
	"dash/data/repo"
	"dash/domain/validation"
	"fmt"
)

type CreateUserCategoryInput struct {
	DisplayName string `validate:"required"`
	IsShelved   bool
}

type CreateUserCategory struct {
	DashboardRepo repo.DashboardRepo
	CategoryRepo  repo.CategoryRepo
	Validator     validation.Validator
}

func NewCreateUserCategory(
	dashboardRepo repo.DashboardRepo,
	categoryRepo repo.CategoryRepo,
	validator validation.Validator,
) *CreateUserCategory {
	return &CreateUserCategory{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
		Validator:     validator,
	}
}

func (uc *CreateUserCategory) Execute(ctx context.Context, userId string, in CreateUserCategoryInput) error {
	if err := uc.Validator.Struct(in); err != nil {
		return fmt.Errorf("%w: %s", ErrValidation, validation.Describe(err))
	}

	dashboard, err := uc.DashboardRepo.GetByUserId(ctx, userId)
	if err != nil {
		return err
	}
	if dashboard == nil {
		return ErrDashboardNotFound
	}
	if dashboard.UserId != userId {
		return ErrUserDoesNotOwnDashboard
	}

	return uc.CategoryRepo.Upsert(ctx, &model.Category{
		DashboardID: dashboard.ID,
		DisplayName: in.DisplayName,
		IsShelved:   in.IsShelved,
	})
}
