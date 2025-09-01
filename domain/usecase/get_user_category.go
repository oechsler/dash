package usecase

import (
	"context"
	"dash/data/repo"
	domainmodel "dash/domain/model"
)

type GetUserCategory struct {
	DashboardRepo repo.DashboardRepo
	CategoryRepo  repo.CategoryRepo
}

func NewGetUserCategory(dashboardRepo repo.DashboardRepo, categoryRepo repo.CategoryRepo) *GetUserCategory {
	return &GetUserCategory{DashboardRepo: dashboardRepo, CategoryRepo: categoryRepo}
}

func (uc *GetUserCategory) Execute(ctx context.Context, userId string, categoryId uint) (*domainmodel.Category, error) {
	dashboard, err := uc.DashboardRepo.GetByUserId(ctx, userId)
	if err != nil {
		return nil, Internal("get user category: get dashboard", err)
	}
	if dashboard == nil {
		return nil, ErrDashboardNotFound
	}

	category, err := uc.CategoryRepo.Get(ctx, categoryId)
	if err != nil {
		return nil, Internal("get user category: get category", err)
	}
	if category == nil {
		return nil, ErrCategoryNotFound
	}
	if category.DashboardID != dashboard.ID {
		return nil, ErrUserDoesNotOwnDashboard
	}

	return &domainmodel.Category{
		ID:          category.ID,
		DisplayName: category.DisplayName,
		IsShelved:   category.IsShelved,
	}, nil
}
