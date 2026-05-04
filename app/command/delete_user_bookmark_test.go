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

func TestDeleteUserBookmark_Handle_ZeroID(t *testing.T) {
	h := command.NewDeleteUserBookmark(nil, nil, nil)
	err := h.Handle(context.Background(), "user-1", 0)

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestDeleteUserBookmark_Handle_BookmarkNotFound(t *testing.T) {
	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("Get", mock.Anything, uint(5)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityBookmark))

	h := command.NewDeleteUserBookmark(nil, nil, bookmarkRepo)
	err := h.Handle(context.Background(), "user-1", 5)

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestDeleteUserBookmark_Handle_ForbiddenWrongOwner(t *testing.T) {
	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.BookmarkRecord{ID: 5, CategoryID: 1}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(1)).
		Return(&domainrepo.CategoryRecord{ID: 1, DashboardID: 99}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	h := command.NewDeleteUserBookmark(dashRepo, catRepo, bookmarkRepo)
	err := h.Handle(context.Background(), "user-1", 5)

	var fe *domainerrors.ForbiddenError
	require.ErrorAs(t, err, &fe)
}

func TestDeleteUserBookmark_Handle_DeleteError(t *testing.T) {
	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.BookmarkRecord{ID: 5, CategoryID: 1}, nil)
	bookmarkRepo.On("Delete", mock.Anything, uint(5)).Return(errors.New("db error"))

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(1)).
		Return(&domainrepo.CategoryRecord{ID: 1, DashboardID: 10}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	h := command.NewDeleteUserBookmark(dashRepo, catRepo, bookmarkRepo)
	err := h.Handle(context.Background(), "user-1", 5)

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

func TestDeleteUserBookmark_Handle_Success(t *testing.T) {
	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.BookmarkRecord{ID: 5, CategoryID: 1}, nil)
	bookmarkRepo.On("Delete", mock.Anything, uint(5)).Return(nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(1)).
		Return(&domainrepo.CategoryRecord{ID: 1, DashboardID: 10}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	h := command.NewDeleteUserBookmark(dashRepo, catRepo, bookmarkRepo)
	err := h.Handle(context.Background(), "user-1", 5)

	require.NoError(t, err)
	bookmarkRepo.AssertExpectations(t)
}
