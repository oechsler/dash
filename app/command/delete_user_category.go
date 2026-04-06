package command

import (
	"context"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// UserCategoryDeleter handles the delete-user-category command.
type UserCategoryDeleter interface {
	Handle(ctx context.Context, userId string, id uint) error
}

type DeleteUserCategory struct {
	DashboardRepo domainrepo.DashboardRepository
	CategoryRepo  domainrepo.CategoryRepository
}

func NewDeleteUserCategory(
	dashboardRepo domainrepo.DashboardRepository,
	categoryRepo domainrepo.CategoryRepository,
) *DeleteUserCategory {
	return &DeleteUserCategory{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
	}
}

func (h *DeleteUserCategory) Handle(ctx context.Context, userId string, id uint) error {
	if id == 0 {
		return domainerrors.Validation(domainerrors.Violation{Message: "id is required"})
	}

	catRecord, err := h.CategoryRepo.Get(ctx, id)
	if err != nil {
		return domainerrors.WrapRepo("delete user category: get category", err)
	}

	dashRecord, err := h.DashboardRepo.GetByUserID(ctx, userId)
	if err != nil {
		return domainerrors.WrapRepo("delete user category: get dashboard", err)
	}
	dash := domainmodel.NewUserDashboard(dashRecord.ID, dashRecord.UserID)
	if !dash.OwnsCategory(catRecord.DashboardID) {
		return domainerrors.Forbidden("user does not own dashboard")
	}

	if err := h.CategoryRepo.Delete(ctx, id); err != nil {
		return domainerrors.Internal("delete user category: delete", err)
	}
	return nil
}
