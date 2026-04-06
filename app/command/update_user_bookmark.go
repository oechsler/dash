package command

import (
	"context"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainmodel "github.com/oechsler-it/dash/domain/model"
	domainrepo "github.com/oechsler-it/dash/domain/repo"

	"github.com/oechsler-it/dash/app/validation"
)

// UpdateUserBookmarkCmd is the input for updating an existing bookmark.
type UpdateUserBookmarkCmd struct {
	ID          uint   `validate:"required,gt=0"`
	Icon        string `validate:"required"`
	DisplayName string `validate:"required"`
	Url         string `validate:"required,url"`
	CategoryID  uint   `validate:"required,gt=0"`
}

// UserBookmarkUpdater handles the UpdateUserBookmarkCmd command.
type UserBookmarkUpdater interface {
	Handle(ctx context.Context, userId string, in UpdateUserBookmarkCmd) error
}

type UpdateUserBookmark struct {
	DashboardRepo domainrepo.DashboardRepository
	CategoryRepo  domainrepo.CategoryRepository
	BookmarkRepo  domainrepo.BookmarkRepository
	Validator     validation.Validator
}

func NewUpdateUserBookmark(
	dashboardRepo domainrepo.DashboardRepository,
	categoryRepo domainrepo.CategoryRepository,
	bookmarkRepo domainrepo.BookmarkRepository,
	validator validation.Validator,
) *UpdateUserBookmark {
	return &UpdateUserBookmark{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
		BookmarkRepo:  bookmarkRepo,
		Validator:     validator,
	}
}

func (h *UpdateUserBookmark) Handle(ctx context.Context, userId string, in UpdateUserBookmarkCmd) error {
	if err := h.Validator.Struct(in); err != nil {
		return domainerrors.Validation(validation.ToViolations(err)...)
	}
	icon, err := domainmodel.ParseIcon(in.Icon)
	if err != nil {
		return domainerrors.Validation(domainerrors.Violation{Message: err.Error()})
	}
	bUrl, err := domainmodel.ParseBookmarkURL(in.Url)
	if err != nil {
		return domainerrors.Validation(domainerrors.Violation{Message: err.Error()})
	}

	bookmarkRecord, err := h.BookmarkRepo.Get(ctx, in.ID)
	if err != nil {
		return domainerrors.WrapRepo("update user bookmark: get bookmark", err)
	}

	currentCatRecord, err := h.CategoryRepo.Get(ctx, bookmarkRecord.CategoryID)
	if err != nil {
		return domainerrors.WrapRepo("update user bookmark: get current category", err)
	}

	dashRecord, err := h.DashboardRepo.GetByUserID(ctx, userId)
	if err != nil {
		return domainerrors.WrapRepo("update user bookmark: get dashboard", err)
	}
	dash := domainmodel.NewUserDashboard(dashRecord.ID, dashRecord.UserID)
	if !dash.OwnsCategory(currentCatRecord.DashboardID) {
		return domainerrors.Forbidden("user does not own dashboard")
	}

	if in.CategoryID != bookmarkRecord.CategoryID {
		targetCatRecord, err := h.CategoryRepo.Get(ctx, in.CategoryID)
		if err != nil {
			return domainerrors.WrapRepo("update user bookmark: get target category", err)
		}
		targetDashRecord, err := h.DashboardRepo.Get(ctx, targetCatRecord.DashboardID)
		if err != nil {
			return domainerrors.WrapRepo("update user bookmark: get target dashboard", err)
		}
		targetDash := domainmodel.NewUserDashboard(targetDashRecord.ID, targetDashRecord.UserID)
		if !targetDash.OwnsCategory(targetCatRecord.DashboardID) {
			return domainerrors.Forbidden("user does not own target dashboard")
		}
	}

	bookmark := domainmodel.Bookmark{
		ID:          bookmarkRecord.ID,
		CategoryID:  bookmarkRecord.CategoryID,
		DisplayName: bookmarkRecord.DisplayName,
	}
	bookmark.UpdateIcon(icon)
	bookmark.Rename(in.DisplayName)
	bookmark.ChangeURL(bUrl)
	bookmark.MoveTo(in.CategoryID)

	if err := h.BookmarkRepo.Upsert(ctx, &domainrepo.BookmarkRecord{
		ID:          bookmark.ID,
		CategoryID:  bookmark.CategoryID,
		Icon:        bookmark.Icon.String(),
		DisplayName: bookmark.DisplayName,
		Url:         bookmark.Url.String(),
	}); err != nil {
		return domainerrors.Internal("update user bookmark: upsert", err)
	}
	return nil
}
