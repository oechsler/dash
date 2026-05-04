package query_test

import (
	"context"
	"testing"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/query"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

func TestGetUserBookmark_Handle_DashboardNotFound(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntityDashboard))

	h := query.NewGetUserBookmark(dashRepo, nil, nil)
	_, err := h.Handle(context.Background(), "user-1", 5)

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestGetUserBookmark_Handle_BookmarkNotFound(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("Get", mock.Anything, uint(5)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityBookmark))

	h := query.NewGetUserBookmark(dashRepo, bookmarkRepo, nil)
	_, err := h.Handle(context.Background(), "user-1", 5)

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestGetUserBookmark_Handle_ForbiddenWrongOwner(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.BookmarkRecord{ID: 5, CategoryID: 1}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(1)).
		Return(&domainrepo.CategoryRecord{ID: 1, DashboardID: 99}, nil)

	h := query.NewGetUserBookmark(dashRepo, bookmarkRepo, catRepo)
	_, err := h.Handle(context.Background(), "user-1", 5)

	var fe *domainerrors.ForbiddenError
	require.ErrorAs(t, err, &fe)
}

func TestGetUserBookmark_Handle_Success(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("Get", mock.Anything, uint(5)).Return(&domainrepo.BookmarkRecord{
		ID:          5,
		CategoryID:  1,
		Icon:        "mdi:link",
		DisplayName: "GitHub",
		Url:         "https://github.com",
	}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(1)).
		Return(&domainrepo.CategoryRecord{ID: 1, DashboardID: 10}, nil)

	h := query.NewGetUserBookmark(dashRepo, bookmarkRepo, catRepo)
	b, err := h.Handle(context.Background(), "user-1", 5)

	require.NoError(t, err)
	require.Equal(t, "GitHub", b.DisplayName)
	require.Equal(t, uint(5), b.ID)
}
