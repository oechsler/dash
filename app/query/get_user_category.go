package query

import (
	"context"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainmodel "github.com/oechsler-it/dash/domain/model"
	domainrepo "github.com/oechsler-it/dash/domain/repo"
)

// UserCategoryGetter handles the get-user-category query.
type UserCategoryGetter interface {
	Handle(ctx context.Context, userId string, categoryId uint) (*domainmodel.Category, error)
}

type GetUserCategory struct {
	DashboardRepo domainrepo.DashboardRepository
	CategoryRepo  domainrepo.CategoryRepository
}

func NewGetUserCategory(dashboardRepo domainrepo.DashboardRepository, categoryRepo domainrepo.CategoryRepository) *GetUserCategory {
	return &GetUserCategory{DashboardRepo: dashboardRepo, CategoryRepo: categoryRepo}
}

func (h *GetUserCategory) Handle(ctx context.Context, userId string, categoryId uint) (*domainmodel.Category, error) {
	dashRecord, err := h.DashboardRepo.GetByUserID(ctx, userId)
	if err != nil {
		return nil, domainerrors.WrapRepo("get user category: get dashboard", err)
	}

	catRecord, err := h.CategoryRepo.Get(ctx, categoryId)
	if err != nil {
		return nil, domainerrors.WrapRepo("get user category: get category", err)
	}

	dash := domainmodel.NewUserDashboard(dashRecord.ID, dashRecord.UserID)
	if !dash.OwnsCategory(catRecord.DashboardID) {
		return nil, domainerrors.Forbidden("user does not own dashboard")
	}

	return &domainmodel.Category{
		ID:          catRecord.ID,
		DisplayName: catRecord.DisplayName,
		IsShelved:   catRecord.IsShelved,
	}, nil
}
