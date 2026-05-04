package query_test

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/query"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

func TestGetUserDashboard_Handle_ExistingDashboard(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("ListByDashboardID", mock.Anything, uint(10)).Return([]domainrepo.CategoryRecord{}, nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("ListByCategoryIDs", mock.Anything, mock.Anything).
		Return([]domainrepo.BookmarkRecord{}, nil)

	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("List", mock.Anything).Return([]domainrepo.ApplicationRecord{}, nil)

	getUserCats := query.NewGetUserCategories(dashRepo, catRepo, bookmarkRepo)
	listApps := query.NewListApplications(appRepo)
	getUserApps := query.NewGetUserApplications(listApps)

	h := query.NewGetUserDashboard(dashRepo, getUserCats, getUserApps)
	dash, err := h.Handle(context.Background(), "user-1", []string{}, "Sam", time.Now())

	require.NoError(t, err)
	require.NotNil(t, dash)
}

func TestGetUserDashboard_Handle_AutoProvisionsDashboard(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	// first call: not found → triggers upsert
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntityDashboard)).Once()
	dashRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.DashboardRecord) bool {
		return r.UserID == "user-1"
	})).Return(nil)
	// second call (inside GetUserCategories): also not found → returns empty
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntityDashboard)).Once()

	catRepo := &repoMock.CategoryRepository{}
	bookmarkRepo := &repoMock.BookmarkRepository{}
	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("List", mock.Anything).Return([]domainrepo.ApplicationRecord{}, nil)

	getUserCats := query.NewGetUserCategories(dashRepo, catRepo, bookmarkRepo)
	listApps := query.NewListApplications(appRepo)
	getUserApps := query.NewGetUserApplications(listApps)

	h := query.NewGetUserDashboard(dashRepo, getUserCats, getUserApps)
	dash, err := h.Handle(context.Background(), "user-1", []string{}, "Sam", time.Now())

	require.NoError(t, err)
	require.NotNil(t, dash)
	dashRepo.AssertCalled(t, "Upsert", mock.Anything, mock.Anything)
}

func TestGetUserDashboard_Handle_DashboardRepoError(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, errors.New("db error"))

	h := query.NewGetUserDashboard(dashRepo, nil, nil)
	_, err := h.Handle(context.Background(), "user-1", []string{}, "Sam", time.Now())

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}
