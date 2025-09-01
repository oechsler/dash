package usecase

import (
	"context"
	"dash/data/repo"
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
		return ValidationMsg("id is required")
	}

	cat, err := uc.CategoryRepo.Get(ctx, id)
	if err != nil {
		return Internal("delete user category: get category", err)
	}
	if cat == nil {
		return ErrCategoryNotFound
	}

	// Efficient ownership check: verify the category's dashboard belongs to the user
	dashboard, err := uc.DashboardRepo.GetByUserId(ctx, userId)
	if err != nil {
		return Internal("delete user category: get dashboard", err)
	}
	if dashboard == nil {
		return ErrDashboardNotFound
	}
	if dashboard.ID != cat.DashboardID {
		return ErrUserDoesNotOwnDashboard
	}

	if err := uc.CategoryRepo.Delete(ctx, id); err != nil {
		return Internal("delete user category: delete", err)
	}
	return nil
}
