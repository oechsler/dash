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

func TestDeleteUserCategory_Handle_ZeroID(t *testing.T) {
	h := command.NewDeleteUserCategory(nil, nil)
	err := h.Handle(context.Background(), "user-1", 0)

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestDeleteUserCategory_Handle_CategoryNotFound(t *testing.T) {
	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(5)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityCategory))

	h := command.NewDeleteUserCategory(nil, catRepo)
	err := h.Handle(context.Background(), "user-1", 5)

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestDeleteUserCategory_Handle_ForbiddenWrongOwner(t *testing.T) {
	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.CategoryRecord{ID: 5, DashboardID: 99}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	h := command.NewDeleteUserCategory(dashRepo, catRepo)
	err := h.Handle(context.Background(), "user-1", 5)

	var fe *domainerrors.ForbiddenError
	require.ErrorAs(t, err, &fe)
}

func TestDeleteUserCategory_Handle_DeleteError(t *testing.T) {
	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.CategoryRecord{ID: 5, DashboardID: 10}, nil)
	catRepo.On("Delete", mock.Anything, uint(5)).Return(errors.New("db error"))

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	h := command.NewDeleteUserCategory(dashRepo, catRepo)
	err := h.Handle(context.Background(), "user-1", 5)

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

func TestDeleteUserCategory_Handle_Success(t *testing.T) {
	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.CategoryRecord{ID: 5, DashboardID: 10}, nil)
	catRepo.On("Delete", mock.Anything, uint(5)).Return(nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	h := command.NewDeleteUserCategory(dashRepo, catRepo)
	err := h.Handle(context.Background(), "user-1", 5)

	require.NoError(t, err)
	catRepo.AssertExpectations(t)
}
