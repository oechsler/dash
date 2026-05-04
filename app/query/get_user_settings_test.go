package query_test

import (
	"context"
	"errors"
	"testing"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/query"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

func TestGetUserSettings_Handle_SettingRepoError(t *testing.T) {
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, errors.New("db error"))

	h := query.NewGetUserSettings(settingRepo, nil)
	_, err := h.Handle(context.Background(), "user-1")

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

func TestGetUserSettings_Handle_Provisioned_WhenNotFound(t *testing.T) {
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntitySetting)).Once()
	settingRepo.On("Upsert", mock.Anything, mock.Anything).Return(nil)
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1", ThemeID: 0}, nil).Once()

	themeRepo := &repoMock.ThemeRepository{}

	h := query.NewGetUserSettings(settingRepo, themeRepo)
	s, err := h.Handle(context.Background(), "user-1")

	require.NoError(t, err)
	require.Equal(t, uint(0), s.ThemeID)
}

func TestGetUserSettings_Handle_DefaultTheme(t *testing.T) {
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1", ThemeID: 0}, nil)

	themeRepo := &repoMock.ThemeRepository{}

	h := query.NewGetUserSettings(settingRepo, themeRepo)
	s, err := h.Handle(context.Background(), "user-1")

	require.NoError(t, err)
	require.Equal(t, uint(0), s.ThemeID)
	themeRepo.AssertNotCalled(t, "GetByID")
}

func TestGetUserSettings_Handle_StoredTheme(t *testing.T) {
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1", ThemeID: 3}, nil)

	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(3)).Return(&domainrepo.ThemeRecord{
		ID: 3, DisplayName: "Custom", Primary: "#111", Secondary: "#222", Tertiary: "#333",
	}, nil)

	h := query.NewGetUserSettings(settingRepo, themeRepo)
	s, err := h.Handle(context.Background(), "user-1")

	require.NoError(t, err)
	require.Equal(t, uint(3), s.ThemeID)
}

func TestGetUserSettings_Handle_StoredTheme_IsSyntheticDuplicate_Normalised(t *testing.T) {
	d := domainmodel.DefaultTheme()
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1", ThemeID: 3}, nil)

	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(3)).Return(&domainrepo.ThemeRecord{
		ID: 3, DisplayName: d.Name, Primary: d.Primary, Secondary: d.Secondary, Tertiary: d.Tertiary,
	}, nil)

	h := query.NewGetUserSettings(settingRepo, themeRepo)
	s, err := h.Handle(context.Background(), "user-1")

	require.NoError(t, err)
	require.Equal(t, uint(0), s.ThemeID) // normalised to 0
}

func TestGetUserSettings_Handle_ThemeDeletedFallsBackToDefault(t *testing.T) {
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1", ThemeID: 3}, nil)

	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(3)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityTheme))

	h := query.NewGetUserSettings(settingRepo, themeRepo)
	s, err := h.Handle(context.Background(), "user-1")

	require.NoError(t, err)
	require.Equal(t, uint(0), s.ThemeID) // fall back to synthetic default
}

func TestGetUserSettings_Handle_LanguageAndTimezone(t *testing.T) {
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1", ThemeID: 0, Language: "de", Timezone: "Europe/Vienna"}, nil)

	themeRepo := &repoMock.ThemeRepository{}

	h := query.NewGetUserSettings(settingRepo, themeRepo)
	s, err := h.Handle(context.Background(), "user-1")

	require.NoError(t, err)
	require.Equal(t, "de", s.Language)
	require.Equal(t, "Europe/Vienna", s.Timezone)
}
