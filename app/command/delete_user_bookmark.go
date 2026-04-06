package command

import (
	"context"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// UserBookmarkDeleter handles the delete-user-bookmark command.
type UserBookmarkDeleter interface {
	Handle(ctx context.Context, userId string, id uint) error
}

type DeleteUserBookmark struct {
	DashboardRepo domainrepo.DashboardRepository
	CategoryRepo  domainrepo.CategoryRepository
	BookmarkRepo  domainrepo.BookmarkRepository
}

func NewDeleteUserBookmark(
	dashboardRepo domainrepo.DashboardRepository,
	categoryRepo domainrepo.CategoryRepository,
	bookmarkRepo domainrepo.BookmarkRepository,
) *DeleteUserBookmark {
	return &DeleteUserBookmark{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
		BookmarkRepo:  bookmarkRepo,
	}
}

func (h *DeleteUserBookmark) Handle(ctx context.Context, userId string, id uint) error {
	if id == 0 {
		return domainerrors.Validation(domainerrors.Violation{Message: "id is required"})
	}

	bookmarkRecord, err := h.BookmarkRepo.Get(ctx, id)
	if err != nil {
		return domainerrors.WrapRepo("delete user bookmark: get bookmark", err)
	}

	catRecord, err := h.CategoryRepo.Get(ctx, bookmarkRecord.CategoryID)
	if err != nil {
		return domainerrors.WrapRepo("delete user bookmark: get category", err)
	}

	dashRecord, err := h.DashboardRepo.GetByUserID(ctx, userId)
	if err != nil {
		return domainerrors.WrapRepo("delete user bookmark: get dashboard", err)
	}
	dash := domainmodel.NewUserDashboard(dashRecord.ID, dashRecord.UserID)
	if !dash.OwnsCategory(catRecord.DashboardID) {
		return domainerrors.Forbidden("user does not own dashboard")
	}

	if err := h.BookmarkRepo.Delete(ctx, id); err != nil {
		return domainerrors.Internal("delete user bookmark: delete", err)
	}
	return nil
}
