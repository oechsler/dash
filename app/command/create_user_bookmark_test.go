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

func validBookmarkCmd() command.CreateUserBookmarkCmd {
	return command.CreateUserBookmarkCmd{
		Icon:        "mdi:link",
		DisplayName: "GitHub",
		Url:         "https://github.com",
		CategoryID:  1,
	}
}

func TestCreateUserBookmark_Handle_ValidationError(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(errors.New("validation failed"))

	h := command.NewCreateUserBookmark(nil, nil, nil, v)
	err := h.Handle(context.Background(), "user-1", command.CreateUserBookmarkCmd{})

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestCreateUserBookmark_Handle_InvalidIcon(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	cmd := validBookmarkCmd()
	cmd.Icon = "not-an-icon"

	h := command.NewCreateUserBookmark(nil, nil, nil, v)
	err := h.Handle(context.Background(), "user-1", cmd)

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestCreateUserBookmark_Handle_CategoryNotFound(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(1)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityCategory))

	h := command.NewCreateUserBookmark(nil, catRepo, nil, v)
	err := h.Handle(context.Background(), "user-1", validBookmarkCmd())

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestCreateUserBookmark_Handle_ForbiddenWrongDashboard(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(1)).
		Return(&domainrepo.CategoryRecord{ID: 1, DashboardID: 99}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	h := command.NewCreateUserBookmark(dashRepo, catRepo, nil, v)
	err := h.Handle(context.Background(), "user-1", validBookmarkCmd())

	var fe *domainerrors.ForbiddenError
	require.ErrorAs(t, err, &fe)
}

func TestCreateUserBookmark_Handle_Success(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(1)).
		Return(&domainrepo.CategoryRecord{ID: 1, DashboardID: 10}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.BookmarkRecord) bool {
		return r.CategoryID == 1 && r.DisplayName == "GitHub"
	})).Return(nil)

	h := command.NewCreateUserBookmark(dashRepo, catRepo, bookmarkRepo, v)
	err := h.Handle(context.Background(), "user-1", validBookmarkCmd())

	require.NoError(t, err)
	bookmarkRepo.AssertExpectations(t)
}

func TestCreateUserBookmark_Handle_UpsertError(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("Get", mock.Anything, uint(1)).
		Return(&domainrepo.CategoryRecord{ID: 1, DashboardID: 10}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("Upsert", mock.Anything, mock.Anything).Return(errors.New("db error"))

	h := command.NewCreateUserBookmark(dashRepo, catRepo, bookmarkRepo, v)
	err := h.Handle(context.Background(), "user-1", validBookmarkCmd())

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}
