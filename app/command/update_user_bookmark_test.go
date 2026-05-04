package command_test

import (
	"context"
	"errors"
	"testing"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

func validUpdateBookmarkCmd() command.UpdateUserBookmarkCmd {
	return command.UpdateUserBookmarkCmd{
		ID:          7,
		Icon:        "mdi:link",
		DisplayName: "Updated",
		Url:         "https://updated.example.com",
		CategoryID:  1,
	}
}

func TestUpdateUserBookmark_Handle_ValidationError(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(errors.New("failed"))

	h := command.NewUpdateUserBookmark(nil, nil, nil, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserBookmarkCmd{})

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestUpdateUserBookmark_Handle_InvalidIcon(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	cmd := validUpdateBookmarkCmd()
	cmd.Icon = "bad-icon"

	h := command.NewUpdateUserBookmark(nil, nil, nil, v)
	err := h.Handle(context.Background(), "user-1", cmd)

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestUpdateUserBookmark_Handle_InvalidURL(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	cmd := validUpdateBookmarkCmd()
	cmd.Url = "not-a-url"

	h := command.NewUpdateUserBookmark(nil, nil, nil, v)
	err := h.Handle(context.Background(), "user-1", cmd)

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestUpdateUserBookmark_Handle_BookmarkNotFound(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("Get", mock.Anything, uint(7)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityBookmark))

	h := command.NewUpdateUserBookmark(nil, nil, bookmarkRepo, v)
	err := h.Handle(context.Background(), "user-1", validUpdateBookmarkCmd())

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestUpdateUserBookmark_Handle_ForbiddenWrongOwner(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("Get", mock.Anything, uint(7)).
		Return(&domainrepo.BookmarkRecord{ID: 7, CategoryID: 1}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(1)).
		Return(&domainrepo.CategoryRecord{ID: 1, DashboardID: 99}, nil) // belongs to dash 99

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil) // owns dash 10

	h := command.NewUpdateUserBookmark(dashRepo, catRepo, bookmarkRepo, v)
	err := h.Handle(context.Background(), "user-1", validUpdateBookmarkCmd())

	var fe *domainerrors.ForbiddenError
	require.ErrorAs(t, err, &fe)
}

func TestUpdateUserBookmark_Handle_Success(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("Get", mock.Anything, uint(7)).
		Return(&domainrepo.BookmarkRecord{ID: 7, CategoryID: 1, DisplayName: "Old", Icon: "mdi:link"}, nil)
	bookmarkRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.BookmarkRecord) bool {
		return r.ID == 7 && r.DisplayName == "Updated"
	})).Return(nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(1)).
		Return(&domainrepo.CategoryRecord{ID: 1, DashboardID: 10}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	h := command.NewUpdateUserBookmark(dashRepo, catRepo, bookmarkRepo, v)
	err := h.Handle(context.Background(), "user-1", validUpdateBookmarkCmd())

	require.NoError(t, err)
	bookmarkRepo.AssertExpectations(t)
}

func TestUpdateUserBookmark_Handle_CrossCategoryMove(t *testing.T) {
	// Moving bookmark to a different category (CategoryID differs from bookmark's current one)
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	cmd := validUpdateBookmarkCmd()
	cmd.CategoryID = 2 // move from cat 1 to cat 2

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("Get", mock.Anything, uint(7)).
		Return(&domainrepo.BookmarkRecord{ID: 7, CategoryID: 1}, nil)
	bookmarkRepo.On("Upsert", mock.Anything, mock.Anything).Return(nil)

	catRepo := &repoMock.CategoryRepository{}
	// current category lookup
	catRepo.On("Get", mock.Anything, uint(1)).
		Return(&domainrepo.CategoryRecord{ID: 1, DashboardID: 10}, nil)
	// target category lookup
	catRepo.On("Get", mock.Anything, uint(2)).
		Return(&domainrepo.CategoryRecord{ID: 2, DashboardID: 10}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)
	// target dashboard lookup
	dashRepo.On("Get", mock.Anything, uint(10)).
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	h := command.NewUpdateUserBookmark(dashRepo, catRepo, bookmarkRepo, v)
	err := h.Handle(context.Background(), "user-1", cmd)

	require.NoError(t, err)
}
