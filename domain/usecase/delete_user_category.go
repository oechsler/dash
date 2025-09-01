package usecase

import (
	"context"
	"dash/data/repo"
	"fmt"
)

// DeleteUserCategory deletes a category owned by the user.
type DeleteUserCategory struct {
	DashboardRepo repo.DashboardRepo
	CategoryRepo  repo.CategoryRepo
}

func NewDeleteUserCategory(
	dashboardRepo repo.DashboardRepo,
	categoryRepo repo.CategoryRepo,
) *DeleteUserCategory {
	return &DeleteUserCategory{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
	}
}

func (uc *DeleteUserCategory) Execute(ctx context.Context, userId string, id uint) error {
	if id == 0 {
		return fmt.Errorf("%w: %s", ErrValidation, "id is required")
	}

	cat, err := uc.CategoryRepo.Get(ctx, id)
	if err != nil {
		return err
	}
	if cat == nil {
		return ErrCategoryNotFound
	}

	// Efficient ownership check: verify the category's dashboard belongs to the user
	dashboard, err := uc.DashboardRepo.GetByUserId(ctx, userId)
	if err != nil {
		return err
	}
	if dashboard == nil {
		return ErrDashboardNotFound
	}
	if dashboard.ID != cat.DashboardID {
		return ErrUserDoesNotOwnDashboard
	}

	return uc.CategoryRepo.Delete(ctx, id)
}
