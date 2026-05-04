package command_test

import (
	"context"
	"errors"
	"testing"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	"git.at.oechsler.it/samuel/dash/v2/app/transfer"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

func emptyExport() *transfer.UserDataExport {
	return &transfer.UserDataExport{Version: 1}
}

func newImportHandler(
	dashRepo *repoMock.DashboardRepository,
	catRepo *repoMock.CategoryRepository,
	bRepo *repoMock.BookmarkRepository,
	themeRepo *repoMock.ThemeRepository,
	settingRepo *repoMock.SettingRepository,
	appRepo *repoMock.ApplicationRepository,
) *command.ImportUserData {
	return command.NewImportUserData(dashRepo, catRepo, bRepo, themeRepo, settingRepo, appRepo)
}

func TestImportUserData_Handle_ListThemesError(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return(nil, errors.New("db error"))

	h := newImportHandler(nil, nil, nil, themeRepo, nil, nil)
	err := h.Handle(context.Background(), "user-1", false, emptyExport())

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

func TestImportUserData_Handle_DashboardRepoError(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").Return(nil, errors.New("db error"))

	h := newImportHandler(dashRepo, nil, nil, themeRepo, nil, nil)
	err := h.Handle(context.Background(), "user-1", false, emptyExport())

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

func TestImportUserData_Handle_EmptyExport_NoDashboard(t *testing.T) {
	// No dashboard yet — must provision one
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntityDashboard)).Once()
	dashRepo.On("Upsert", mock.Anything, mock.Anything).Return(nil)
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil).Once()

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(nil, domainerrors.NotFound(domainerrors.EntitySetting))
	settingRepo.On("Upsert", mock.Anything, mock.Anything).Return(nil)

	h := newImportHandler(dashRepo, nil, nil, themeRepo, settingRepo, nil)
	err := h.Handle(context.Background(), "user-1", false, emptyExport())

	require.NoError(t, err)
}

func TestImportUserData_Handle_WithExistingDashboard_HappyPath(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("ListByDashboardID", mock.Anything, uint(10)).Return([]domainrepo.CategoryRecord{}, nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1"}, nil)
	settingRepo.On("Upsert", mock.Anything, mock.Anything).Return(nil)

	h := newImportHandler(dashRepo, catRepo, nil, themeRepo, settingRepo, nil)
	err := h.Handle(context.Background(), "user-1", false, emptyExport())

	require.NoError(t, err)
}

func TestImportUserData_Handle_ImportNewCategory(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("ListByDashboardID", mock.Anything, uint(10)).Return([]domainrepo.CategoryRecord{}, nil)
	catRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.CategoryRecord) bool {
		return r.DisplayName == "Work"
	})).Run(func(args mock.Arguments) {
		// Simulate GORM's Save() setting the ID on the record pointer
		rec := args.Get(1).(*domainrepo.CategoryRecord)
		rec.ID = 5
	}).Return(nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("ListByCategoryIDs", mock.Anything, []uint{5}).
		Return([]domainrepo.BookmarkRecord{}, nil)
	bookmarkRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.BookmarkRecord) bool {
		return r.DisplayName == "GitHub"
	})).Return(nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1"}, nil)
	settingRepo.On("Upsert", mock.Anything, mock.Anything).Return(nil)

	in := &transfer.UserDataExport{
		Version: 1,
		Categories: []transfer.CategoryExport{
			{
				Hash:        transfer.ContentHash("Work", "false"),
				DisplayName: "Work",
				IsShelved:   false,
				Bookmarks: []transfer.BookmarkExport{
					{
						Hash:        transfer.ContentHash("mdi:link", "GitHub", "https://github.com"),
						Icon:        "mdi:link",
						DisplayName: "GitHub",
						URL:         "https://github.com",
					},
				},
			},
		},
	}

	h := newImportHandler(dashRepo, catRepo, bookmarkRepo, themeRepo, settingRepo, nil)
	err := h.Handle(context.Background(), "user-1", false, in)

	require.NoError(t, err)
	catRepo.AssertExpectations(t)
	bookmarkRepo.AssertExpectations(t)
}

func TestImportUserData_Handle_AdminImportsApplications(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("ListByDashboardID", mock.Anything, uint(10)).Return([]domainrepo.CategoryRecord{}, nil)

	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("List", mock.Anything).Return([]domainrepo.ApplicationRecord{}, nil)
	appRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.ApplicationRecord) bool {
		return r.DisplayName == "Proxmox"
	})).Return(nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1"}, nil)
	settingRepo.On("Upsert", mock.Anything, mock.Anything).Return(nil)

	in := &transfer.UserDataExport{
		Version: 1,
		Applications: []transfer.ApplicationExport{
			{
				Hash:        transfer.ContentHash("mdi:server", "Proxmox", "https://proxmox.local", ""),
				Icon:        "mdi:server",
				DisplayName: "Proxmox",
				URL:         "https://proxmox.local",
			},
		},
	}

	h := newImportHandler(dashRepo, catRepo, nil, themeRepo, settingRepo, appRepo)
	err := h.Handle(context.Background(), "user-1", true, in)

	require.NoError(t, err)
	appRepo.AssertExpectations(t)
}

func TestImportUserData_Handle_SkipsDuplicateCategory(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{}, nil)

	dashRepo := &repoMock.DashboardRepository{}
	dashRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.DashboardRecord{ID: 10, UserID: "user-1"}, nil)

	// Dashboard already has "Work" (same hash)
	catRepo := &repoMock.CategoryRepository{}
	catRepo.On("ListByDashboardID", mock.Anything, uint(10)).Return([]domainrepo.CategoryRecord{
		{ID: 5, DashboardID: 10, DisplayName: "Work", IsShelved: false},
	}, nil)

	bookmarkRepo := &repoMock.BookmarkRepository{}
	bookmarkRepo.On("ListByCategoryIDs", mock.Anything, mock.Anything).
		Return([]domainrepo.BookmarkRecord{}, nil)

	settingRepo := &repoMock.SettingRepository{}
	settingRepo.On("GetByUserID", mock.Anything, "user-1").
		Return(&domainrepo.SettingRecord{UserID: "user-1"}, nil)
	settingRepo.On("Upsert", mock.Anything, mock.Anything).Return(nil)

	in := &transfer.UserDataExport{
		Version: 1,
		Categories: []transfer.CategoryExport{
			{
				Hash:        transfer.ContentHash("Work", "false"),
				DisplayName: "Work",
				IsShelved:   false,
				Bookmarks:   []transfer.BookmarkExport{},
			},
		},
	}

	h := newImportHandler(dashRepo, catRepo, bookmarkRepo, themeRepo, settingRepo, nil)
	err := h.Handle(context.Background(), "user-1", false, in)

	require.NoError(t, err)
	catRepo.AssertNotCalled(t, "Upsert")
}
