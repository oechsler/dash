package app

import (
	"git.at.oechsler.it/samuel/dash/v2/app/command"
	"git.at.oechsler.it/samuel/dash/v2/app/query"
	"git.at.oechsler.it/samuel/dash/v2/app/validation"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// Repos declares the repository dependencies the application layer needs.
// Fields are domain interfaces — the application layer does not know about infra.
type Repos struct {
	User        domainrepo.UserRepository
	Dashboard   domainrepo.DashboardRepository
	Category    domainrepo.CategoryRepository
	Bookmark    domainrepo.BookmarkRepository
	Application domainrepo.ApplicationRepository
	Setting     domainrepo.SettingRepository
	Theme       domainrepo.ThemeRepository
	Session         domainrepo.SessionRepository
	UserIDMigration domainrepo.UserIDMigrationRepository
	IdpLink         domainrepo.IdpLinkRepository
}

// UseCases bundles all use cases exposed to the delivery layer.
// All fields are interfaces so the delivery layer depends on abstractions only.
type UseCases struct {
	// Queries
	ExportUserData           query.UserDataExporter
	GetUserDashboard         query.UserDashboardGetter
	GetUserSettings          query.UserSettingsGetter
	GetUserThemeByID         query.UserThemeByIDGetter
	GetUserApplications      query.UserApplicationsGetter
	ListApplications         query.ApplicationsLister
	GetApplication           query.ApplicationGetter
	GetAvailableIconTypes    query.AvailableIconTypesGetter
	GetUserCategories        query.UserCategoriesGetter
	GetUserShelvedCategories query.UserShelvedCategoriesGetter
	GetUserCategory          query.UserCategoryGetter
	GetUserBookmark          query.UserBookmarkGetter
	ListUserThemes           query.UserThemesLister
	// Session use cases
	GetSessionsOverview query.UserSessionsOverviewGetter
	CreateSession       command.SessionCreator
	RefreshSession      command.SessionRefresher
	PinSession          command.SessionPinner
	UnpinSession        command.SessionUnpinner
	InvalidateSession   command.SessionInvalidator
	TerminateSession    command.SessionTerminator
	CleanupSessions     command.SessionCleaner
	MigrateUserID          command.UserIDMigrator
	ResolveOrCreateUser    command.UserResolver
	// Commands
	DeleteUserData     command.UserDataDeleter
	ImportUserData     command.UserDataImporter
	EnsureDefaultTheme command.DefaultThemeEnsurer
	UpdateUserSettings command.UserSettingsUpdater
	CreateUserTheme    command.UserThemeCreator
	DeleteUserTheme    command.UserThemeDeleter
	CreateApplication  command.ApplicationCreator
	UpdateApplication  command.ApplicationUpdater
	DeleteApplication  command.ApplicationDeleter
	CreateUserCategory command.UserCategoryCreator
	UpdateUserCategory command.UserCategoryUpdater
	DeleteUserCategory command.UserCategoryDeleter
	CreateUserBookmark command.UserBookmarkCreator
	UpdateUserBookmark command.UserBookmarkUpdater
	DeleteUserBookmark command.UserBookmarkDeleter
}

func NewUseCases(repos Repos, v validation.Validator) *UseCases {
	listApplications := query.NewListApplications(repos.Application)
	getUserApplications := query.NewGetUserApplications(listApplications)
	getApplication := query.NewGetApplication(repos.Application)

	getUserCategories := query.NewGetUserCategories(repos.Dashboard, repos.Category, repos.Bookmark)
	getUserCategory := query.NewGetUserCategory(repos.Dashboard, repos.Category)
	getUserShelvedCategories := query.NewGetUserShelvedCategories(repos.Dashboard, repos.Category, repos.Bookmark)

	getUserBookmark := query.NewGetUserBookmark(repos.Dashboard, repos.Bookmark, repos.Category)

	getUserDashboard := query.NewGetUserDashboard(repos.Dashboard, getUserCategories, getUserApplications)

	listUserThemes := query.NewListUserThemes(repos.Theme, repos.Setting)
	getUserThemeByID := query.NewGetUserThemeByID(repos.Theme)
	getAvailableIconTypes := query.NewGetAvailableIconTypes()

	ensureDefaultTheme := command.NewEnsureDefaultTheme(repos.Theme)
	getUserSettings := query.NewGetUserSettings(repos.Setting, repos.Theme, ensureDefaultTheme)

	getSessionsOverview := query.NewGetSessionsOverview(repos.Session)
	createSession := command.NewCreateSession(repos.Session)
	refreshSession := command.NewRefreshSession(repos.Session)
	pinSession := command.NewPinSession(repos.Session)
	unpinSession := command.NewUnpinSession(repos.Session)
	invalidateSession := command.NewInvalidateSession(repos.Session)
	terminateSession := command.NewTerminateSession(repos.Session)
	cleanupSessions := command.NewCleanupSessions(repos.Session)
	migrateUserID := command.NewMigrateUserID(repos.UserIDMigration)
	resolveOrCreateUser := command.NewResolveOrCreateUser(repos.IdpLink)

	exportUserData := query.NewExportUserData(repos.Dashboard, repos.Category, repos.Bookmark, repos.Theme, repos.Setting, repos.Application)
	deleteUserData := command.NewDeleteUserData(repos.User)
	importUserData := command.NewImportUserData(repos.Dashboard, repos.Category, repos.Bookmark, repos.Theme, repos.Setting, repos.Application, ensureDefaultTheme)

	return &UseCases{
		GetSessionsOverview:      getSessionsOverview,
		CreateSession:            createSession,
		RefreshSession:           refreshSession,
		PinSession:               pinSession,
		UnpinSession:             unpinSession,
		InvalidateSession:        invalidateSession,
		TerminateSession:         terminateSession,
		CleanupSessions:          cleanupSessions,
		MigrateUserID:            migrateUserID,
		ResolveOrCreateUser:      resolveOrCreateUser,
		ExportUserData:           exportUserData,
		DeleteUserData:           deleteUserData,
		ImportUserData:           importUserData,
		GetUserDashboard:         getUserDashboard,
		GetUserSettings:          getUserSettings,
		GetUserThemeByID:         getUserThemeByID,
		GetUserApplications:      getUserApplications,
		ListApplications:         listApplications,
		GetApplication:           getApplication,
		GetAvailableIconTypes:    getAvailableIconTypes,
		GetUserCategories:        getUserCategories,
		GetUserShelvedCategories: getUserShelvedCategories,
		GetUserCategory:          getUserCategory,
		GetUserBookmark:          getUserBookmark,
		ListUserThemes:           listUserThemes,
		EnsureDefaultTheme:       ensureDefaultTheme,
		UpdateUserSettings:       command.NewUpdateUserSettings(repos.Setting, repos.Theme, v),
		CreateUserTheme:          command.NewCreateUserTheme(repos.Theme, v),
		DeleteUserTheme:          command.NewDeleteUserTheme(repos.Theme, repos.Setting),
		CreateApplication:        command.NewCreateApplication(repos.Application, v),
		UpdateApplication:        command.NewUpdateApplication(repos.Application, v),
		DeleteApplication:        command.NewDeleteApplication(repos.Application),
		CreateUserCategory:       command.NewCreateUserCategory(repos.Dashboard, repos.Category, v),
		UpdateUserCategory:       command.NewUpdateUserCategory(repos.Dashboard, repos.Category, v),
		DeleteUserCategory:       command.NewDeleteUserCategory(repos.Dashboard, repos.Category),
		CreateUserBookmark:       command.NewCreateUserBookmark(repos.Dashboard, repos.Category, repos.Bookmark, v),
		UpdateUserBookmark:       command.NewUpdateUserBookmark(repos.Dashboard, repos.Category, repos.Bookmark, v),
		DeleteUserBookmark:       command.NewDeleteUserBookmark(repos.Dashboard, repos.Category, repos.Bookmark),
	}
}
