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

func newExportHandler(
	dashRepo *repoMock.DashboardRepository,
	catRepo *repoMock.CategoryRepository,
	bRepo *repoMock.BookmarkRepository,
	themeRepo *repoMock.ThemeRepository,
	settingRepo *repoMock.SettingRepository,
	appRepo *repoMock.ApplicationRepository,
) *query.ExportUserData {
	return query.NewExportUserData(dashRepo, catRepo, bRepo, themeRepo, settingRepo, appRepo)
}

func TestExportUserData_Handle_SettingsRepoError(t *testing.T) {
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").Return(nil, errors.New("db error"))

	h := newExportHandler(nil, nil, nil, nil, settingRepo, nil)
	_, err := h.Handle(context.Background(), "user-1", "sam", false)

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

func TestExportUserData_Handle_ThemeRepoError(t *testing.T) {
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntitySetting))

	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return(nil, errors.New("db error"))

	h := newExportHandler(nil, nil, nil, themeRepo, settingRepo, nil)
	_, err := h.Handle(context.Background(), "user-1", "sam", false)

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

func TestExportUserData_Handle_NoDashboard_ReturnsEmptyCategories(t *testing.T) {
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntitySetting))

	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntityDashboard))

	h := newExportHandler(dashRepo, nil, nil, themeRepo, settingRepo, nil)
	export, err := h.Handle(context.Background(), "user-1", "sam", false)

	require.NoError(t, err)
	require.Empty(t, export.Categories)
	require.Empty(t, export.Themes)
	require.Equal(t, "sam", export.Username)
}

func TestExportUserData_Handle_WithCategoriesAndBookmarks(t *testing.T) {
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1", Language: "de", Timezone: "Europe/Vienna"}, nil)

	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{
		{ID: 2, UserID: "user-1", DisplayName: "Custom", Primary: "#111", Secondary: "#222", Tertiary: "#333"},
	}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("ListByDashboardID", mock.Anything, uint(10)).Return([]domainrepo.CategoryRecord{
		{ID: 1, DashboardID: 10, DisplayName: "Work", IsShelved: false},
	}, nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("ListByCategoryIDs", mock.Anything, []uint{1}).Return([]domainrepo.BookmarkRecord{
		{ID: 10, CategoryID: 1, Icon: "mdi:link", DisplayName: "GitHub", Url: "https://github.com"},
	}, nil)

	h := newExportHandler(dashRepo, catRepo, bookmarkRepo, themeRepo, settingRepo, nil)
	export, err := h.Handle(context.Background(), "user-1", "sam", false)

	require.NoError(t, err)
	require.Len(t, export.Categories, 1)
	require.Equal(t, "Work", export.Categories[0].DisplayName)
	require.Len(t, export.Categories[0].Bookmarks, 1)
	require.Equal(t, "GitHub", export.Categories[0].Bookmarks[0].DisplayName)
	require.Len(t, export.Themes, 1)
	require.Equal(t, "Custom", export.Themes[0].Name)
	require.Equal(t, "de", export.Settings.Language)
}

func TestExportUserData_Handle_AdminExportsApplications(t *testing.T) {
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntitySetting))

	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{}, nil)

	// Admin must have a dashboard to reach the applications section
	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("ListByDashboardID", mock.Anything, uint(10)).Return([]domainrepo.CategoryRecord{}, nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("ListByCategoryIDs", mock.Anything, mock.Anything).
		Return([]domainrepo.BookmarkRecord{}, nil)

	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("List", mock.Anything).Return([]domainrepo.ApplicationRecord{
		{ID: 1, Icon: "mdi:server", DisplayName: "Proxmox", Url: "https://proxmox.local"},
	}, nil)

	h := newExportHandler(dashRepo, catRepo, bookmarkRepo, themeRepo, settingRepo, appRepo)
	export, err := h.Handle(context.Background(), "user-1", "sam", true)

	require.NoError(t, err)
	require.Len(t, export.Applications, 1)
	require.Equal(t, "Proxmox", export.Applications[0].DisplayName)
}

func TestExportUserData_Handle_FiltersSyntheticDuplicateThemes(t *testing.T) {
	d := domainmodel.DefaultTheme()
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntitySetting))

	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{
		// synthetic duplicate — should be filtered
		{ID: 1, DisplayName: d.Name, Primary: d.Primary, Secondary: d.Secondary, Tertiary: d.Tertiary},
		// real user theme — should be kept
		{ID: 2, DisplayName: "Custom", Primary: "#111", Secondary: "#222", Tertiary: "#333"},
	}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntityDashboard))

	h := newExportHandler(dashRepo, nil, nil, themeRepo, settingRepo, nil)
	export, err := h.Handle(context.Background(), "user-1", "sam", false)

	require.NoError(t, err)
	require.Len(t, export.Themes, 1)
	require.Equal(t, "Custom", export.Themes[0].Name)
}

func TestExportUserData_Handle_ActiveThemeName(t *testing.T) {
	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1", ThemeID: 2}, nil)

	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{
		{ID: 2, DisplayName: "My Theme", Primary: "#aaa", Secondary: "#bbb", Tertiary: "#ccc"},
	}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntityDashboard))

	h := newExportHandler(dashRepo, nil, nil, themeRepo, settingRepo, nil)
	export, err := h.Handle(context.Background(), "user-1", "sam", false)

	require.NoError(t, err)
	require.Equal(t, "My Theme", export.Settings.ThemeName)
}
