package usecase

import (
	"context"
	"dash/data/model"
	"dash/data/repo"
	"dash/domain/validation"
)

type CreateUserBookmarkInput struct {
	Icon        string `validate:"required"`
	DisplayName string `validate:"required"`
	Url         string `validate:"required,url"`
	CategoryID  uint   `validate:"required,gt=0"`
}

type CreateUserBookmark struct {
	DashboardRepo repo.DashboardRepo
	CategoryRepo  repo.CategoryRepo
	BookmarkRepo  repo.BookmarkRepo
	Validator     validation.Validator
}

func NewCreazeUserBookmark(
	dashboardRepo repo.DashboardRepo,
	categoryRepo repo.CategoryRepo,
	bookmarkRepo repo.BookmarkRepo,
	validator validation.Validator,
) *CreateUserBookmark {
	return &CreateUserBookmark{
		DashboardRepo: dashboardRepo,
		CategoryRepo:  categoryRepo,
		BookmarkRepo:  bookmarkRepo,
		Validator:     validator,
	}
}

func (uc *CreateUserBookmark) Execute(ctx context.Context, userId string, in CreateUserBookmarkInput) error {
	if err := uc.Validator.Struct(in); err != nil {
		return Validation(err)
	}

	cat, err := uc.CategoryRepo.Get(ctx, in.CategoryID)
	if err != nil {
		return Internal("create user bookmark: get category", err)
	}
	if cat == nil {
		return ErrCategoryNotFound
	}

	dashboard, err := uc.DashboardRepo.GetByUserId(ctx, userId)
	if err != nil {
		return Internal("create user bookmark: get dashboard", err)
	}
	if dashboard == nil {
		return ErrDashboardNotFound
	}
	if dashboard.ID != cat.DashboardID {
		return ErrUserDoesNotOwnDashboard
	}

	if err := uc.BookmarkRepo.Upsert(ctx, &model.Bookmark{
		CategoryID:  in.CategoryID,
		Icon:        in.Icon,
		DisplayName: in.DisplayName,
		Url:         in.Url,
	}); err != nil {
		return Internal("create user bookmark: upsert", err)
	}
	return nil
}
