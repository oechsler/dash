package query

import (
	"context"
	"errors"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainmodel "github.com/oechsler-it/dash/domain/model"
	domainrepo "github.com/oechsler-it/dash/domain/repo"
	"github.com/samber/lo"
)

// UserShelvedCategoriesGetter handles the get-user-shelved-categories query.
type UserShelvedCategoriesGetter interface {
	Handle(ctx context.Context, userId string) ([]domainmodel.Category, error)
}

type GetUserShelvedCategories struct {
	DashboardRepo domainrepo.DashboardRepository
	CategoryRepo  domainrepo.CategoryRepository
	BookmarkRepo  domainrepo.BookmarkRepository
}

func NewGetUserShelvedCategories(
	dashboardRepo domainrepo.DashboardRepository,
	categoryRepo domainrepo.CategoryRepository,
	bookmarkRepo domainrepo.BookmarkRepository,
) *GetUserShelvedCategories {
	return &GetUserShelvedCategories{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
		BookmarkRepo:  bookmarkRepo,
	}
}

func (h *GetUserShelvedCategories) Handle(ctx context.Context, userId string) ([]domainmodel.Category, error) {
	dashboard, err := h.DashboardRepo.GetByUserID(ctx, userId)
	if err != nil {
		var nfe *domainerrors.NotFoundError
		if errors.As(err, &nfe) {
			return []domainmodel.Category{}, nil
		}
		return nil, domainerrors.Internal("get user shelved categories: get dashboard", err)
	}

	categories, err := h.CategoryRepo.ListByDashboardID(ctx, dashboard.ID)
	if err != nil {
		return nil, domainerrors.Internal("get user shelved categories: list categories", err)
	}

	shelvedCategories := lo.Filter(categories, func(category domainrepo.CategoryRecord, _ int) bool {
		return category.IsShelved
	})

	categoryIDs := lo.Map(shelvedCategories, func(category domainrepo.CategoryRecord, _ int) uint {
		return category.ID
	})

	dataBookmarks, err := h.BookmarkRepo.ListByCategoryIDs(ctx, categoryIDs)
	if err != nil {
		return nil, err
	}

	domainBookmarks := make([]domainmodel.Bookmark, 0, len(dataBookmarks))
	for _, b := range dataBookmarks {
		icon, err := domainmodel.ParseIcon(b.Icon)
		if err != nil {
			return nil, domainerrors.Internal("get user shelved categories: parse icon", err)
		}
		bUrl, err := domainmodel.ParseBookmarkURL(b.Url)
		if err != nil {
			return nil, domainerrors.Internal("get user shelved categories: parse url", err)
		}
		domainBookmarks = append(domainBookmarks, domainmodel.Bookmark{
			ID:          b.ID,
			Icon:        icon,
			DisplayName: b.DisplayName,
			Url:         bUrl,
			CategoryID:  b.CategoryID,
		})
	}

	bookmarksByCategory := lo.GroupBy(domainBookmarks, func(bookmark domainmodel.Bookmark) uint {
		return bookmark.CategoryID
	})

	result := make([]domainmodel.Category, 0, len(shelvedCategories))
	for _, category := range shelvedCategories {
		bookmarksOfCategory := bookmarksByCategory[category.ID]
		result = append(result, domainmodel.Category{
			ID:          category.ID,
			DisplayName: category.DisplayName,
			IsShelved:   category.IsShelved,
			Bookmarks:   bookmarksOfCategory,
		})
	}
	return result, nil
}
