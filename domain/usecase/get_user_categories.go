package usecase

import (
	"context"
	"dash/data/model"
	"dash/data/repo"
	domainmodel "dash/domain/model"

	"github.com/samber/lo"
)

type GetUserCategories struct {
	DashboardRepo repo.DashboardRepo
	CategoryRepo  repo.CategoryRepo
	BookmarkRepo  repo.BookmarkRepo
}

func NewGetUserCategories(
	dashboardRepo repo.DashboardRepo,
	categoryRepo repo.CategoryRepo,
	bookmarkRepo repo.BookmarkRepo,
) *GetUserCategories {
	return &GetUserCategories{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
		BookmarkRepo:  bookmarkRepo,
	}
}

func (uc *GetUserCategories) Execute(ctx context.Context, userId string) ([]domainmodel.Category, error) {
	dashboard, err := uc.DashboardRepo.GetByUserId(ctx, userId)
	if err != nil {
		return nil, Internal("get user categories: get dashboard", err)
	}
	if dashboard == nil {
		return []domainmodel.Category{}, nil
	}

	categories, err := uc.CategoryRepo.ListByDashboardID(ctx, dashboard.ID)
	if err != nil {
		return nil, Internal("get user categories: list categories", err)
	}

	notShelvedCategories := lo.Filter(categories, func(category model.Category, _ int) bool {
		return !category.IsShelved
	})

	categoryIDs := lo.Map(notShelvedCategories, func(category model.Category, _ int) uint {
		return category.ID
	})

	dataBookmarks, err := uc.BookmarkRepo.ListByCategoryIDs(ctx, categoryIDs)
	if err != nil {
		return nil, err
	}

	domainBookmarks := make([]domainmodel.Bookmark, 0, len(dataBookmarks))
	for _, bookmark := range dataBookmarks {
		domainBookmarks = append(domainBookmarks, domainmodel.Bookmark{
			ID:          bookmark.ID,
			Icon:        bookmark.Icon,
			DisplayName: bookmark.DisplayName,
			Url:         bookmark.Url,
			CategoryID:  bookmark.CategoryID,
		})
	}

	bookmarksByCategory := lo.GroupBy(domainBookmarks, func(bookmark domainmodel.Bookmark) uint {
		return bookmark.CategoryID
	})

	result := make([]domainmodel.Category, 0, len(notShelvedCategories))
	for _, category := range notShelvedCategories {
		bookmarksOfCategory := bookmarksByCategory[category.ID]
		result = append(result, domainmodel.Category{
			ID:          category.ID,
			DisplayName: category.DisplayName,
			Bookmarks:   bookmarksOfCategory,
		})
	}
	return result, nil
}
