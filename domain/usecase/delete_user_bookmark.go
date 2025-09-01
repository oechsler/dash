package usecase

import (
	"context"
	"dash/data/repo"
)

type DeleteUserBookmark struct {
	DashboardRepo repo.DashboardRepo
	CategoryRepo  repo.CategoryRepo
	BookmarkRepo  repo.BookmarkRepo
}

func NewDeleteUserBookmark(
	dashboardRepo repo.DashboardRepo,
	categoryRepo repo.CategoryRepo,
	bookmarkRepo repo.BookmarkRepo,
) *DeleteUserBookmark {
	return &DeleteUserBookmark{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
		BookmarkRepo:  bookmarkRepo,
	}
}

func (uc *DeleteUserBookmark) Execute(ctx context.Context, userId string, id uint) error {
	if id == 0 {
		return ValidationMsg("id is required")
	}

	bookmark, err := uc.BookmarkRepo.Get(ctx, id)
	if err != nil {
		return Internal("delete user bookmark: get bookmark", err)
	}
	if bookmark == nil {
		return ErrBookmarkNotFound
	}

	category, err := uc.CategoryRepo.Get(ctx, bookmark.CategoryID)
	if err != nil {
		return Internal("delete user bookmark: get category", err)
	}
	if category == nil {
		return ErrCategoryNotFound
	}

	dashboard, err := uc.DashboardRepo.GetByUserId(ctx, userId)
	if err != nil {
		return Internal("delete user bookmark: get dashboard", err)
	}
	if dashboard == nil {
		return ErrDashboardNotFound
	}
	if dashboard.ID != category.DashboardID {
		return ErrUserDoesNotOwnDashboard
	}

	if err := uc.BookmarkRepo.Delete(ctx, id); err != nil {
		return Internal("delete user bookmark: delete", err)
	}
	return nil
}
