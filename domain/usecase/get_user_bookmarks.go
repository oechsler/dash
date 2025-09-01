package usecase

import (
	"context"
	"dash/data/repo"
	domainmodel "dash/domain/model"
)

type GetUserBookmarks struct {
	DashboardRepo repo.DashboardRepo
	BookmarkRepo  repo.BookmarkRepo
}

func NewGetUserBookmarks(
	dashboardRepo repo.DashboardRepo,
	bookmarkRepo repo.BookmarkRepo,
) *GetUserBookmarks {
	return &GetUserBookmarks{DashboardRepo: dashboardRepo, BookmarkRepo: bookmarkRepo}
}

func (uc *GetUserBookmarks) Execute(ctx context.Context, userId string) ([]domainmodel.Bookmark, error) {
	dashboard, err := uc.DashboardRepo.GetByUserId(ctx, userId)
	if err != nil {
		return nil, err
	}
	if dashboard == nil {
		return []domainmodel.Bookmark{}, nil
	}

	bookmarks, err := uc.BookmarkRepo.ListByDashboardID(ctx, dashboard.ID)
	if err != nil {
		return nil, err
	}

	result := make([]domainmodel.Bookmark, 0, len(bookmarks))
	for _, bookmark := range bookmarks {
		result = append(result, domainmodel.Bookmark{
			ID:          bookmark.ID,
			Icon:        bookmark.Icon,
			DisplayName: bookmark.DisplayName,
			Description: bookmark.Description,
			Url:         bookmark.Url,
			CategoryID:  bookmark.CategoryID,
		})
	}
	return result, nil
}
