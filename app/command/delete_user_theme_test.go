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

func TestDeleteUserTheme_Handle_NotFound_IsIgnored(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(5)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityTheme))

	h := command.NewDeleteUserTheme(themeRepo, nil)
	err := h.Handle(context.Background(), "user-1", 5)

	require.NoError(t, err)
}

func TestDeleteUserTheme_Handle_GetByIDRepoError(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(5)).
		Return(nil, errors.New("db error"))

	h := command.NewDeleteUserTheme(themeRepo, nil)
	err := h.Handle(context.Background(), "user-1", 5)

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

func TestDeleteUserTheme_Handle_ForbiddenActiveTheme(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(5)).
		Return(&domainrepo.ThemeRecord{ID: 5}, nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{ThemeID: 5}, nil) // theme 5 is active

	h := command.NewDeleteUserTheme(themeRepo, settingRepo)
	err := h.Handle(context.Background(), "user-1", 5)

	var fe *domainerrors.ForbiddenError
	require.ErrorAs(t, err, &fe)
}

func TestDeleteUserTheme_Handle_SettingsNotFound_StillDeletes(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(5)).
		Return(&domainrepo.ThemeRecord{ID: 5}, nil)
	themeRepo.On("Delete", mock.Anything, "user-1", uint(5)).Return(nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntitySetting))

	h := command.NewDeleteUserTheme(themeRepo, settingRepo)
	err := h.Handle(context.Background(), "user-1", 5)

	require.NoError(t, err)
	themeRepo.AssertExpectations(t)
}

func TestDeleteUserTheme_Handle_SettingsRepoError(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(5)).
		Return(&domainrepo.ThemeRecord{ID: 5}, nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, errors.New("db error"))

	h := command.NewDeleteUserTheme(themeRepo, settingRepo)
	err := h.Handle(context.Background(), "user-1", 5)

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

func TestDeleteUserTheme_Handle_Success(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(5)).
		Return(&domainrepo.ThemeRecord{ID: 5}, nil)
	themeRepo.On("Delete", mock.Anything, "user-1", uint(5)).Return(nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{ThemeID: 3}, nil) // different active theme

	h := command.NewDeleteUserTheme(themeRepo, settingRepo)
	err := h.Handle(context.Background(), "user-1", 5)

	require.NoError(t, err)
	themeRepo.AssertExpectations(t)
}
