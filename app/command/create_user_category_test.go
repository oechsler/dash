package command_test

import (
	"context"
	"errors"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

func TestCreateUserCategory_Handle_ValidationError(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(errors.New("validation failed"))

	h := command.NewCreateUserCategory(nil, nil, v)
	err := h.Handle(context.Background(), "user-1", command.CreateUserCategoryCmd{})

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestCreateUserCategory_Handle_DashboardNotFound(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntityDashboard))

	h := command.NewCreateUserCategory(dashRepo, nil, v)
	err := h.Handle(context.Background(), "user-1", command.CreateUserCategoryCmd{DisplayName: "Work"})

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestCreateUserCategory_Handle_DashboardRepoError(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, errors.New("db error"))

	h := command.NewCreateUserCategory(dashRepo, nil, v)
	err := h.Handle(context.Background(), "user-1", command.CreateUserCategoryCmd{DisplayName: "Work"})

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

func TestCreateUserCategory_Handle_Success(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.CategoryRecord) bool {
		return r.DashboardID == 10 && r.DisplayName == "Work" && !r.IsShelved
	})).Return(nil)

	h := command.NewCreateUserCategory(dashRepo, catRepo, v)
	err := h.Handle(context.Background(), "user-1", command.CreateUserCategoryCmd{DisplayName: "Work"})

	require.NoError(t, err)
	catRepo.AssertExpectations(t)
}

func TestCreateUserCategory_Handle_Shelved(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.CategoryRecord) bool {
		return r.IsShelved
	})).Return(nil)

	h := command.NewCreateUserCategory(dashRepo, catRepo, v)
	err := h.Handle(context.Background(), "user-1", command.CreateUserCategoryCmd{DisplayName: "Archive", IsShelved: true})

	require.NoError(t, err)
}

func TestCreateUserCategory_Handle_UpsertError(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Upsert", mock.Anything, mock.Anything).Return(errors.New("db error"))

	h := command.NewCreateUserCategory(dashRepo, catRepo, v)
	err := h.Handle(context.Background(), "user-1", command.CreateUserCategoryCmd{DisplayName: "Work"})

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)

	_ = assert.Error // satisfy import
}
