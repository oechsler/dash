package command

import (
	"context"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainmodel "github.com/oechsler-it/dash/domain/model"
	domainrepo "github.com/oechsler-it/dash/domain/repo"

	"github.com/oechsler-it/dash/app/validation"
)

// CreateUserBookmarkCmd is the input for creating a new bookmark.
type CreateUserBookmarkCmd struct {
	Icon        string `validate:"required"`
	DisplayName string `validate:"required"`
	Url         string `validate:"required,url"`
	CategoryID  uint   `validate:"required,gt=0"`
}

// UserBookmarkCreator handles the CreateUserBookmarkCmd command.
type UserBookmarkCreator interface {
	Handle(ctx context.Context, userId string, in CreateUserBookmarkCmd) error
}

type CreateUserBookmark struct {
	DashboardRepo domainrepo.DashboardRepository
	CategoryRepo  domainrepo.CategoryRepository
	BookmarkRepo  domainrepo.BookmarkRepository
	Validator     validation.Validator
}

func NewCreateUserBookmark(
	dashboardRepo domainrepo.DashboardRepository,
	categoryRepo domainrepo.CategoryRepository,
	bookmarkRepo domainrepo.BookmarkRepository,
	validator validation.Validator,
) *CreateUserBookmark {
	return &CreateUserBookmark{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
		BookmarkRepo:  bookmarkRepo,
		Validator:     validator,
	}
}

func (h *CreateUserBookmark) Handle(ctx context.Context, userId string, in CreateUserBookmarkCmd) error {
	if err := h.Validator.Struct(in); err != nil {
		return domainerrors.Validation(validation.ToViolations(err)...)
	}
	if _, err := domainmodel.ParseIcon(in.Icon); err != nil {
		return domainerrors.Validation(domainerrors.Violation{Message: err.Error()})
	}

	catRecord, err := h.CategoryRepo.Get(ctx, in.CategoryID)
	if err != nil {
		return domainerrors.WrapRepo("create user bookmark: get category", err)
	}

	dashRecord, err := h.DashboardRepo.GetByUserID(ctx, userId)
	if err != nil {
		return domainerrors.WrapRepo("create user bookmark: get dashboard", err)
	}
	dash := domainmodel.NewUserDashboard(dashRecord.ID, dashRecord.UserID)
	if !dash.OwnsCategory(catRecord.DashboardID) {
		return domainerrors.Forbidden("user does not own dashboard")
	}

	if err := h.BookmarkRepo.Upsert(ctx, &domainrepo.BookmarkRecord{
		CategoryID:  in.CategoryID,
		Icon:        in.Icon,
		DisplayName: in.DisplayName,
		Url:         in.Url,
	}); err != nil {
		return domainerrors.Internal("create user bookmark: upsert", err)
	}
	return nil
}
