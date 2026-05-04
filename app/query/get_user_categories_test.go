package query_test

import (
	"context"
	"errors"
	"testing"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/query"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

func TestGetUserCategories_Handle_NoDashboard_ReturnsEmpty(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntityDashboard))

	h := query.NewGetUserCategories(dashRepo, nil, nil)
	cats, err := h.Handle(context.Background(), "user-1")

	require.NoError(t, err)
	require.Empty(t, cats)
}

func TestGetUserCategories_Handle_DashboardRepoError(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, errors.New("db error"))

	h := query.NewGetUserCategories(dashRepo, nil, nil)
	_, err := h.Handle(context.Background(), "user-1")

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

func TestGetUserCategories_Handle_NoCategories(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("ListByDashboardID", mock.Anything, uint(10)).
		Return([]domainrepo.CategoryRecord{}, nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("ListByCategoryIDs", mock.Anything, mock.Anything).
		Return([]domainrepo.BookmarkRecord{}, nil)

	h := query.NewGetUserCategories(dashRepo, catRepo, bookmarkRepo)
	cats, err := h.Handle(context.Background(), "user-1")

	require.NoError(t, err)
	require.Empty(t, cats)
}

func TestGetUserCategories_Handle_ShelvedFiltered(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("ListByDashboardID", mock.Anything, uint(10)).Return([]domainrepo.CategoryRecord{
		{ID: 1, DashboardID: 10, DisplayName: "Work", IsShelved: false},
		{ID: 2, DashboardID: 10, DisplayName: "Archive", IsShelved: true},
	}, nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("ListByCategoryIDs", mock.Anything, []uint{1}).
		Return([]domainrepo.BookmarkRecord{}, nil)

	h := query.NewGetUserCategories(dashRepo, catRepo, bookmarkRepo)
	cats, err := h.Handle(context.Background(), "user-1")

	require.NoError(t, err)
	require.Len(t, cats, 1)
	require.Equal(t, "Work", cats[0].DisplayName)
}

func TestGetUserCategories_Handle_WithBookmarks(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("ListByDashboardID", mock.Anything, uint(10)).Return([]domainrepo.CategoryRecord{
		{ID: 1, DashboardID: 10, DisplayName: "Work"},
	}, nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("ListByCategoryIDs", mock.Anything, []uint{1}).Return([]domainrepo.BookmarkRecord{
		{ID: 10, CategoryID: 1, Icon: "mdi:link", DisplayName: "GitHub", Url: "https://github.com"},
	}, nil)

	h := query.NewGetUserCategories(dashRepo, catRepo, bookmarkRepo)
	cats, err := h.Handle(context.Background(), "user-1")

	require.NoError(t, err)
	require.Len(t, cats, 1)
	require.Len(t, cats[0].Bookmarks, 1)
	require.Equal(t, "GitHub", cats[0].Bookmarks[0].DisplayName)
}
