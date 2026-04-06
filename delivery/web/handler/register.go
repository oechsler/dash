package handler

import (
	"github.com/oechsler-it/dash/app"
	"github.com/oechsler-it/dash/delivery/web/middleware"
	"github.com/oechsler-it/dash/infra/oidc"

	"github.com/gofiber/fiber/v2"
)

// BuildInfo holds version metadata injected at build time via ldflags.
type BuildInfo struct {
	Version   string
	Commit    string
	BuildDate string
}

func RegisterAll(
	fiberApp *fiber.App,
	sessionStore *oidc.SessionStore,
	oidcProvider *oidc.Provider,
	uc *app.UseCases,
	buildInfo BuildInfo,
) {
	// Language middleware runs globally — resolves locale from user settings or Accept-Language header
	// and stores it in the request user context for all Templ templates.
	fiberApp.Use(middleware.WithLanguage(sessionStore, uc.GetUserSettings))

	SettingPlain(SettingDeps{
		SessionStore:   sessionStore,
		App:            fiberApp,
		ExportUserData: uc.ExportUserData,
		BuildInfo:      buildInfo,
	})

	Session(fiberApp, oidcProvider, sessionStore)
	Favicon(sessionStore, fiberApp)

	Dashboard(DashboardDeps{
		SessionStore:       sessionStore,
		App:                fiberApp,
		GetUserDashboard:   uc.GetUserDashboard,
		GetUserSettings:    uc.GetUserSettings,
		EnsureDefaultTheme: uc.EnsureDefaultTheme,
		GetUserThemeByID:   uc.GetUserThemeByID,
	})

	Application(ApplicationDeps{
		SessionStore:          sessionStore,
		App:                   fiberApp,
		CreateApplication:     uc.CreateApplication,
		DeleteApplication:     uc.DeleteApplication,
		UpdateApplication:     uc.UpdateApplication,
		GetUserApplications:   uc.GetUserApplications,
		ListApplications:      uc.ListApplications,
		GetApplication:        uc.GetApplication,
		GetAvailableIconTypes: uc.GetAvailableIconTypes,
	})

	Category(CategoryDeps{
		SessionStore:             sessionStore,
		App:                      fiberApp,
		GetUserCategories:        uc.GetUserCategories,
		GetUserShelvedCategories: uc.GetUserShelvedCategories,
		GetUserCategory:          uc.GetUserCategory,
		CategoryCreate:           uc.CreateUserCategory,
		CategoryUpdate:           uc.UpdateUserCategory,
		CategoryDelete:           uc.DeleteUserCategory,
	})

	Bookmark(BookmarkDeps{
		SessionStore:             sessionStore,
		App:                      fiberApp,
		GetUserBookmark:          uc.GetUserBookmark,
		GetUserCategory:          uc.GetUserCategory,
		GetUserCategories:        uc.GetUserCategories,
		GetUserShelvedCategories: uc.GetUserShelvedCategories,
		BookmarkCreate:           uc.CreateUserBookmark,
		BookmarkUpdate:           uc.UpdateUserBookmark,
		BookmarkDelete:           uc.DeleteUserBookmark,
		GetAvailableIconTypes:    uc.GetAvailableIconTypes,
	})

	Setting(SettingDeps{
		SessionStore:       sessionStore,
		App:                fiberApp,
		GetUserSettings:    uc.GetUserSettings,
		UpdateUserSettings: uc.UpdateUserSettings,
		ListUserThemes:     uc.ListUserThemes,
		EnsureDefaultTheme: uc.EnsureDefaultTheme,
		ExportUserData:     uc.ExportUserData,
		DeleteUserData:     uc.DeleteUserData,
		ImportUserData:     uc.ImportUserData,
		BuildInfo:          buildInfo,
	})

	Theme(ThemeDeps{
		SessionStore:    sessionStore,
		App:             fiberApp,
		ListUserThemes:  uc.ListUserThemes,
		CreateUserTheme: uc.CreateUserTheme,
		DeleteUserTheme: uc.DeleteUserTheme,
		GetUserSettings: uc.GetUserSettings,
	})
}
