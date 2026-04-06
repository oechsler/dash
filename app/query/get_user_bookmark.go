package query

import (
	"context"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainmodel "github.com/oechsler-it/dash/domain/model"
	domainrepo "github.com/oechsler-it/dash/domain/repo"
)

// UserBookmarkGetter handles the get-user-bookmark query.
type UserBookmarkGetter interface {
	Handle(ctx context.Context, userId string, bookmarkId uint) (*domainmodel.Bookmark, error)
}

type GetUserBookmark struct {
	DashboardRepo domainrepo.DashboardRepository
	BookmarkRepo  domainrepo.BookmarkRepository
	CategoryRepo  domainrepo.CategoryRepository
}

func NewGetUserBookmark(dashboardRepo domainrepo.DashboardRepository, bookmarkRepo domainrepo.BookmarkRepository, categoryRepo domainrepo.CategoryRepository) *GetUserBookmark {
	return &GetUserBookmark{DashboardRepo: dashboardRepo, BookmarkRepo: bookmarkRepo, CategoryRepo: categoryRepo}
}

func (h *GetUserBookmark) Handle(ctx context.Context, userId string, bookmarkId uint) (*domainmodel.Bookmark, error) {
	dashRecord, err := h.DashboardRepo.GetByUserID(ctx, userId)
	if err != nil {
		return nil, domainerrors.WrapRepo("get user bookmark: get dashboard", err)
	}

	bookmarkRecord, err := h.BookmarkRepo.Get(ctx, bookmarkId)
	if err != nil {
		return nil, domainerrors.WrapRepo("get user bookmark: get bookmark", err)
	}

	catRecord, err := h.CategoryRepo.Get(ctx, bookmarkRecord.CategoryID)
	if err != nil {
		return nil, domainerrors.WrapRepo("get user bookmark: get category", err)
	}

	dash := domainmodel.NewUserDashboard(dashRecord.ID, dashRecord.UserID)
	if !dash.OwnsCategory(catRecord.DashboardID) {
		return nil, domainerrors.Forbidden("user does not own dashboard")
	}

	icon, err := domainmodel.ParseIcon(bookmarkRecord.Icon)
	if err != nil {
		return nil, domainerrors.Internal("get user bookmark: parse icon", err)
	}
	bUrl, err := domainmodel.ParseBookmarkURL(bookmarkRecord.Url)
	if err != nil {
		return nil, domainerrors.Internal("get user bookmark: parse url", err)
	}

	return &domainmodel.Bookmark{
		ID:          bookmarkRecord.ID,
		Icon:        icon,
		DisplayName: bookmarkRecord.DisplayName,
		Url:         bUrl,
		CategoryID:  bookmarkRecord.CategoryID,
	}, nil
}
