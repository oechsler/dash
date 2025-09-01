package usecase

import (
	"context"
	"dash/data/repo"
	"dash/domain/validation"
	"fmt"
)

type UpdateUserBookmarkInput struct {
	ID          uint   `validate:"required,gt=0"`
	Icon        string `validate:"required"`
	DisplayName string `validate:"required"`
	Url         string `validate:"required,url"`
	CategoryID  uint   `validate:"required,gt=0"`
}

type UpdateUserBookmark struct {
	DashboardRepo repo.DashboardRepo
	CategoryRepo  repo.CategoryRepo
	BookmarkRepo  repo.BookmarkRepo
	Validator     validation.Validator
}

func NewUpdateUserBookmark(
	dashboardRepo repo.DashboardRepo,
	categoryRepo repo.CategoryRepo,
	bookmarkRepo repo.BookmarkRepo,
	validator validation.Validator,
) *UpdateUserBookmark {
	return &UpdateUserBookmark{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
		BookmarkRepo:  bookmarkRepo,
		Validator:     validator,
	}
}

func (uc *UpdateUserBookmark) Execute(ctx context.Context, userId string, in UpdateUserBookmarkInput) error {
	if err := uc.Validator.Struct(in); err != nil {
		return Validation(err)
	}

	bookmark, err := uc.BookmarkRepo.Get(ctx, in.ID)
	if err != nil {
		return Internal("update user bookmark: get bookmark", err)
	}
	if bookmark == nil {
		return ErrBookmarkNotFound
	}

	currentCategory, err := uc.CategoryRepo.Get(ctx, bookmark.CategoryID)
	if err != nil {
		return Internal("update user bookmark: get current category", err)
	}
	if currentCategory == nil {
		return ErrCategoryNotFound
	}

	dashboard, err := uc.DashboardRepo.GetByUserId(ctx, userId)
	if err != nil {
		return Internal("update user bookmark: get dashboard", err)
	}
	if dashboard == nil {
		return ErrDashboardNotFound
	}
	if dashboard.ID != currentCategory.DashboardID {
		return ErrUserDoesNotOwnDashboard
	}

	if in.CategoryID != bookmark.CategoryID {
		targetCategory, err := uc.CategoryRepo.Get(ctx, in.CategoryID)
		if err != nil {
			return Internal("update user bookmark: get target category", err)
		}
		if targetCategory == nil {
			return fmt.Errorf("%w: %s", ErrCategoryNotFound, "target category not found")
		}

		targetDashboard, err := uc.DashboardRepo.Get(ctx, targetCategory.DashboardID)
		if err != nil {
			return Internal("update user bookmark: get target dashboard", err)
		}
		if targetDashboard == nil {
			return fmt.Errorf("%w: %s", ErrDashboardNotFound, "target dashboard not found")
		}
		if targetDashboard.ID != targetCategory.DashboardID {
			return fmt.Errorf("%w: %s", ErrUserDoesNotOwnDashboard, "user does not own target dashboard")
		}
	}

	bookmark.Icon = in.Icon
	bookmark.DisplayName = in.DisplayName
	bookmark.Url = in.Url
	bookmark.CategoryID = in.CategoryID

	if err := uc.BookmarkRepo.Upsert(ctx, bookmark); err != nil {
		return Internal("update user bookmark: upsert", err)
	}
	return nil
}
