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

func TestGetUserCategory_Handle_DashboardNotFound(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntityDashboard))

	h := query.NewGetUserCategory(dashRepo, nil)
	_, err := h.Handle(context.Background(), "user-1", 5)

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestGetUserCategory_Handle_CategoryNotFound(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(5)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityCategory))

	h := query.NewGetUserCategory(dashRepo, catRepo)
	_, err := h.Handle(context.Background(), "user-1", 5)

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestGetUserCategory_Handle_ForbiddenWrongOwner(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.CategoryRecord{ID: 5, DashboardID: 99}, nil)

	h := query.NewGetUserCategory(dashRepo, catRepo)
	_, err := h.Handle(context.Background(), "user-1", 5)

	var fe *domainerrors.ForbiddenError
	require.ErrorAs(t, err, &fe)
}

func TestGetUserCategory_Handle_Success(t *testing.T) {
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.CategoryRecord{ID: 5, DashboardID: 10, DisplayName: "Work", IsShelved: false}, nil)

	h := query.NewGetUserCategory(dashRepo, catRepo)
	cat, err := h.Handle(context.Background(), "user-1", 5)

	require.NoError(t, err)
	require.Equal(t, "Work", cat.DisplayName)
	require.Equal(t, uint(5), cat.ID)
}
