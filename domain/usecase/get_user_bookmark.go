package usecase

import (
	"context"
	"dash/data/repo"
	domainmodel "dash/domain/model"
)

type GetUserBookmark struct {
	DashboardRepo repo.DashboardRepo
	BookmarkRepo  repo.BookmarkRepo
	CategoryRepo  repo.CategoryRepo
}

func NewGetUserBookmark(dashboardRepo repo.DashboardRepo, bookmarkRepo repo.BookmarkRepo, categoryRepo repo.CategoryRepo) *GetUserBookmark {
	return &GetUserBookmark{DashboardRepo: dashboardRepo, BookmarkRepo: bookmarkRepo, CategoryRepo: categoryRepo}
}

func (uc *GetUserBookmark) Execute(ctx context.Context, userId string, bookmarkId uint) (*domainmodel.Bookmark, error) {
	dashboard, err := uc.DashboardRepo.GetByUserId(ctx, userId)
	if err != nil {
		return nil, err
	}
	if dashboard == nil {
		return nil, ErrDashboardNotFound
	}

	bookmark, err := uc.BookmarkRepo.Get(ctx, bookmarkId)
	if err != nil {
		return nil, err
	}
	if bookmark == nil {
		return nil, ErrBookmarkNotFound
	}

	category, err := uc.CategoryRepo.Get(ctx, bookmark.CategoryID)
	if err != nil {
		return nil, err
	}
	if category == nil {
		return nil, ErrCategoryNotFound
	}
	if category.DashboardID != dashboard.ID {
		return nil, ErrUserDoesNotOwnDashboard
	}

	return &domainmodel.Bookmark{
		ID:          bookmark.ID,
		Icon:        bookmark.Icon,
		DisplayName: bookmark.DisplayName,
		Description: bookmark.Description,
		Url:         bookmark.Url,
		CategoryID:  bookmark.CategoryID,
	}, nil
}
