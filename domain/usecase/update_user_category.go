package usecase

import (
	"context"
	"dash/data/repo"
	"dash/domain/validation"
	"fmt"
)

type UpdateUserCategoryInput struct {
	ID          uint   `validate:"required,gt=0"`
	DisplayName string `validate:"required"`
	IsShelved   bool
}

type UpdateUserCategory struct {
	DashboardRepo repo.DashboardRepo
	CategoryRepo  repo.CategoryRepo
	Validator     validation.Validator
}

func NewUpdateUserCategory(
	dashboardRepo repo.DashboardRepo,
	categoryRepo repo.CategoryRepo,
	validator validation.Validator,
) *UpdateUserCategory {
	return &UpdateUserCategory{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
		Validator:     validator,
	}
}

func (uc *UpdateUserCategory) Execute(ctx context.Context, userId string, in UpdateUserCategoryInput) error {
	if err := uc.Validator.Struct(in); err != nil {
		return fmt.Errorf("%w: %s", ErrValidation, validation.Describe(err))
	}

	category, err := uc.CategoryRepo.Get(ctx, in.ID)
	if err != nil {
		return err
	}
	if category == nil {
		return ErrCategoryNotFound
	}

	dashboard, err := uc.DashboardRepo.GetByUserId(ctx, userId)
	if err != nil {
		return err
	}
	if dashboard == nil {
		return ErrDashboardNotFound
	}
	if dashboard.ID != category.DashboardID {
		return ErrUserDoesNotOwnDashboard
	}

	category.DisplayName = in.DisplayName
	category.IsShelved = in.IsShelved

	return uc.CategoryRepo.Upsert(ctx, category)
}
