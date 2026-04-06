package command

import (
	"context"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainmodel "github.com/oechsler-it/dash/domain/model"
	domainrepo "github.com/oechsler-it/dash/domain/repo"

	"github.com/oechsler-it/dash/app/validation"
)

// UpdateUserCategoryCmd is the input for updating an existing category.
type UpdateUserCategoryCmd struct {
	ID          uint   `validate:"required,gt=0"`
	DisplayName string `validate:"required"`
	IsShelved   bool
}

// UserCategoryUpdater handles the UpdateUserCategoryCmd command.
type UserCategoryUpdater interface {
	Handle(ctx context.Context, userId string, in UpdateUserCategoryCmd) error
}

type UpdateUserCategory struct {
	DashboardRepo domainrepo.DashboardRepository
	CategoryRepo  domainrepo.CategoryRepository
	Validator     validation.Validator
}

func NewUpdateUserCategory(
	dashboardRepo domainrepo.DashboardRepository,
	categoryRepo domainrepo.CategoryRepository,
	validator validation.Validator,
) *UpdateUserCategory {
	return &UpdateUserCategory{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
		Validator:     validator,
	}
}

func (h *UpdateUserCategory) Handle(ctx context.Context, userId string, in UpdateUserCategoryCmd) error {
	if err := h.Validator.Struct(in); err != nil {
		return domainerrors.Validation(validation.ToViolations(err)...)
	}

	catRecord, err := h.CategoryRepo.Get(ctx, in.ID)
	if err != nil {
		return domainerrors.WrapRepo("update user category: get category", err)
	}

	dashRecord, err := h.DashboardRepo.GetByUserID(ctx, userId)
	if err != nil {
		return domainerrors.WrapRepo("update user category: get dashboard", err)
	}
	dash := domainmodel.NewUserDashboard(dashRecord.ID, dashRecord.UserID)
	if !dash.OwnsCategory(catRecord.DashboardID) {
		return domainerrors.Forbidden("user does not own dashboard")
	}

	cat := domainmodel.Category{
		ID:          catRecord.ID,
		DisplayName: catRecord.DisplayName,
		IsShelved:   catRecord.IsShelved,
	}
	if err := cat.Rename(in.DisplayName); err != nil {
		return domainerrors.Validation(domainerrors.Violation{Message: err.Error()})
	}
	if in.IsShelved {
		cat.Shelve()
	} else {
		cat.Unshelve()
	}

	if err := h.CategoryRepo.Upsert(ctx, &domainrepo.CategoryRecord{
		ID:          cat.ID,
		DashboardID: dash.ID(),
		DisplayName: cat.DisplayName,
		IsShelved:   cat.IsShelved,
	}); err != nil {
		return domainerrors.Internal("update user category: upsert", err)
	}
	return nil
}
