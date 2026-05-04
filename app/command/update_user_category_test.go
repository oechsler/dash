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

func TestUpdateUserCategory_Handle_ValidationError(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(errors.New("validation failed"))

	h := command.NewUpdateUserCategory(nil, nil, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserCategoryCmd{})

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestUpdateUserCategory_Handle_CategoryNotFound(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(5)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityCategory))

	h := command.NewUpdateUserCategory(nil, catRepo, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserCategoryCmd{ID: 5, DisplayName: "New"})

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestUpdateUserCategory_Handle_DashboardNotFound(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.CategoryRecord{ID: 5, DashboardID: 10, DisplayName: "Old"}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntityDashboard))

	h := command.NewUpdateUserCategory(dashRepo, catRepo, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserCategoryCmd{ID: 5, DisplayName: "New"})

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestUpdateUserCategory_Handle_ForbiddenWrongOwner(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.CategoryRecord{ID: 5, DashboardID: 99, DisplayName: "Old"}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil) // owns dash 10, cat belongs to 99

	h := command.NewUpdateUserCategory(dashRepo, catRepo, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserCategoryCmd{ID: 5, DisplayName: "New"})

	var fe *domainerrors.ForbiddenError
	require.ErrorAs(t, err, &fe)
}

func TestUpdateUserCategory_Handle_Success(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.CategoryRecord{ID: 5, DashboardID: 10, DisplayName: "Old"}, nil)
	catRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.CategoryRecord) bool {
		return r.ID == 5 && r.DisplayName == "New" && !r.IsShelved
	})).Return(nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	h := command.NewUpdateUserCategory(dashRepo, catRepo, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserCategoryCmd{ID: 5, DisplayName: "New", IsShelved: false})

	require.NoError(t, err)
	catRepo.AssertExpectations(t)
}

func TestUpdateUserCategory_Handle_Shelve(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.CategoryRecord{ID: 5, DashboardID: 10, DisplayName: "Old"}, nil)
	catRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.CategoryRecord) bool {
		return r.IsShelved
	})).Return(nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	h := command.NewUpdateUserCategory(dashRepo, catRepo, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserCategoryCmd{ID: 5, DisplayName: "Old", IsShelved: true})

	require.NoError(t, err)
}
