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

func TestUpdateUserSettings_Handle_ValidationError(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(errors.New("failed"))

	h := command.NewUpdateUserSettings(nil, nil, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserSettingsCmd{})

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestUpdateUserSettings_Handle_ExistingSettings_DefaultTheme(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1", ThemeID: 1}, nil)
	settingRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.SettingRecord) bool {
		return r.ThemeID == 0 // default theme
	})).Return(nil)

	// ThemeID=0 → default, so ThemeRepo.GetByID is NOT called
	themeRepo := &repoMock.ThemeRepository{}

	h := command.NewUpdateUserSettings(settingRepo, themeRepo, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserSettingsCmd{ThemeID: 0})

	require.NoError(t, err)
	themeRepo.AssertNotCalled(t, "GetByID")
}

func TestUpdateUserSettings_Handle_NewSettings_Provisioned(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntitySetting))
	settingRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.SettingRecord) bool {
		return r.UserID == "user-1" && r.ThemeID == 0
	})).Return(nil)

	themeRepo := &repoMock.ThemeRepository{}

	h := command.NewUpdateUserSettings(settingRepo, themeRepo, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserSettingsCmd{ThemeID: 0})

	require.NoError(t, err)
}

func TestUpdateUserSettings_Handle_NonDefaultTheme_Validated(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1"}, nil)
	settingRepo.On("Upsert", mock.Anything, mock.Anything).Return(nil)

	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(3)).
		Return(&domainrepo.ThemeRecord{ID: 3}, nil)

	h := command.NewUpdateUserSettings(settingRepo, themeRepo, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserSettingsCmd{ThemeID: 3})

	require.NoError(t, err)
	themeRepo.AssertExpectations(t)
}

func TestUpdateUserSettings_Handle_ThemeNotFound(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1"}, nil)

	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(3)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityTheme))

	h := command.NewUpdateUserSettings(settingRepo, themeRepo, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserSettingsCmd{ThemeID: 3})

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestUpdateUserSettings_Handle_Language(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1"}, nil)
	settingRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.SettingRecord) bool {
		return r.Language == "de"
	})).Return(nil)

	themeRepo := &repoMock.ThemeRepository{}

	h := command.NewUpdateUserSettings(settingRepo, themeRepo, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserSettingsCmd{Language: "de"})

	require.NoError(t, err)
}

func TestUpdateUserSettings_Handle_ValidTimezone(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1"}, nil)
	settingRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.SettingRecord) bool {
		return r.Timezone == "Europe/Vienna"
	})).Return(nil)

	themeRepo := &repoMock.ThemeRepository{}

	h := command.NewUpdateUserSettings(settingRepo, themeRepo, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserSettingsCmd{Timezone: "Europe/Vienna"})

	require.NoError(t, err)
}

func TestUpdateUserSettings_Handle_InvalidTimezone(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1"}, nil)

	themeRepo := &repoMock.ThemeRepository{}

	h := command.NewUpdateUserSettings(settingRepo, themeRepo, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserSettingsCmd{Timezone: "Not/ATimezone"})

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestUpdateUserSettings_Handle_AutoTimezone(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1"}, nil)
	settingRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.SettingRecord) bool {
		return r.Timezone == "auto"
	})).Return(nil)

	themeRepo := &repoMock.ThemeRepository{}

	h := command.NewUpdateUserSettings(settingRepo, themeRepo, v)
	err := h.Handle(context.Background(), "user-1", command.UpdateUserSettingsCmd{Timezone: "auto"})

	require.NoError(t, err)
}
