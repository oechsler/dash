//go:generate go run ../genstatic/main.go -out ../../static
package main

import (
	"context"
	"dash/data/repo"
	"dash/domain/usecase"
	"dash/domain/validation"
	"dash/endpoint"
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/gofiber/fiber/v2"
	htmltpl "github.com/gofiber/template/html/v2"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

var (
	app                             *fiber.App
	db                              *gorm.DB
	dashboardRepo                   *repo.GormDashboardRepo
	categoryRepo                    *repo.GormCategoryRepo
	bookmarkRepo                    *repo.GormBookmarkRepo
	applicationRepo                 *repo.GormApplicationRepo
	settingRepo                     *repo.GormSettingRepo
	themeRepo                       *repo.GormThemeRepo
	getUserDashboardUseCase         *usecase.GetUserDashboard
	getUserDashboardGreetingUseCase *usecase.GetUserDashboardGreeting
	getUserApplicationsUseCase      *usecase.GetUserApplications
	getApplicationsUseCase          *usecase.ListApplications
	getApplicationUseCase           *usecase.GetApplication
	getUserCategoriesUseCase        *usecase.GetUserCategories
	getUserShelvedCategoriesUseCase *usecase.GetUserShelvedCategories
	getUserCategoryUseCase          *usecase.GetUserCategory
	getUserBookmarksUseCase         *usecase.GetUserBookmarks
	getUserBookmarkUseCase          *usecase.GetUserBookmark
	getUserSettingsUseCase          *usecase.GetUserSettings
	updateUserSettingsUseCase       *usecase.UpdateUserSettings
	listUserThemesUseCase           *usecase.ListUserThemes
	createUserThemeUseCase          *usecase.CreateUserTheme
	deleteUserThemeUseCase          *usecase.DeleteUserTheme
	ensureDefaultThemeUseCase       *usecase.EnsureDefaultTheme
	getUserThemeByIDUseCase         *usecase.GetUserThemeByID
	applicationCreateUseCase        *usecase.CreateApplication
	applicationDeleteUseCase        *usecase.DeleteApplication
	applicationUpdateUseCase        *usecase.UpdateApplication
	categoryCreateUseCase           *usecase.CreateUserCategory
	categoryUpdateUseCase           *usecase.UpdateUserCategory
	categoryDeleteUseCase           *usecase.DeleteUserCategory
	bookmarkCreateUseCase           *usecase.CreateUserBookmark
	bookmarkUpdateUseCase           *usecase.UpdateUserBookmark
	bookmarkDeleteUseCase           *usecase.DeleteUserBookmark
)

func init() {
	var err error
	db, err = gorm.Open(sqlite.Open("dash.db"), &gorm.Config{
		PrepareStmt: true,
	})
	if err != nil {
		log.Fatal("Failed to connect database")
	}
	log.Println("Connected to database")

	dashboardRepo, err = repo.NewGormDashboardRepo(db)
	if err != nil {
		log.Fatal("Failed to initialize dashboard repo")
	}
	log.Println("Initialized dashboard repo")

	categoryRepo, err = repo.NewGormCategoryRepo(db)
	if err != nil {
		log.Fatal("Failed to initialize category repo")
	}
	log.Println("Initialized category repo")

	bookmarkRepo, err = repo.NewGormBookmarkRepo(db)
	if err != nil {
		log.Fatal("Failed to initialize bookmark repo")
	}
	log.Println("Initialized bookmark repo")

	applicationRepo, err = repo.NewGormApplicationRepo(db)
	if err != nil {
		log.Fatal("Failed to initialize application repo")
	}
	log.Println("Initialized application repo")

	settingRepo, err = repo.NewGormSettingRepo(db)
	if err != nil {
		log.Fatal("Failed to initialize setting repo")
	}
	log.Println("Initialized setting repo")

	themeRepo, err = repo.NewGormThemeRepo(db)
	if err != nil {
		log.Fatal("Failed to initialize theme repo")
	}
	log.Println("Initialized theme repo")

	getApplicationsUseCase = usecase.NewListApplications(applicationRepo)
	log.Println("Initialized applications (list) use case")

	getUserApplicationsUseCase = usecase.NewGetUserApplications(getApplicationsUseCase)
	log.Println("Initialized user applications use case")

	getApplicationUseCase = usecase.NewGetApplication(applicationRepo)
	log.Println("Initialized get application use case")

	getUserBookmarksUseCase = usecase.NewGetUserBookmarks(dashboardRepo, bookmarkRepo)
	log.Println("Initialized user bookmarks (list) use case")

	getUserBookmarkUseCase = usecase.NewGetUserBookmark(dashboardRepo, bookmarkRepo, categoryRepo)
	log.Println("Initialized user bookmark (single) use case")

 getUserCategoriesUseCase = usecase.NewGetUserCategories(dashboardRepo, categoryRepo, bookmarkRepo)
	log.Println("Initialized user categories (list) use case")

 getUserCategoryUseCase = usecase.NewGetUserCategory(dashboardRepo, categoryRepo)
	log.Println("Initialized user category (single) use case")

	getUserShelvedCategoriesUseCase = usecase.NewGetUserShelvedCategories(dashboardRepo, categoryRepo, bookmarkRepo)
	log.Println("Initialized user shelved categories (list) use case")

	getUserDashboardGreetingUseCase = usecase.NewGetUserDashboardGreeting()
	log.Println("Initialized user dashboard greeting use case")

	getUserDashboardUseCase = usecase.NewGetUserDashboard(dashboardRepo, getUserCategoriesUseCase, getUserApplicationsUseCase, getUserDashboardGreetingUseCase)
	log.Println("Initialized user dashboard use case")

	getUserSettingsUseCase = usecase.NewGetUserSettings(settingRepo, themeRepo)
	updateUserSettingsUseCase = usecase.NewUpdateUserSettings(settingRepo, themeRepo)
	listUserThemesUseCase = usecase.NewListUserThemes(themeRepo)
	createUserThemeUseCase = usecase.NewCreateUserTheme(themeRepo)
	deleteUserThemeUseCase = usecase.NewDeleteUserTheme(themeRepo)
	ensureDefaultThemeUseCase = usecase.NewEnsureDefaultTheme(themeRepo)
	getUserThemeByIDUseCase = usecase.NewGetUserThemeByID(themeRepo)
	log.Println("Initialized settings/themes use cases")

	v := validation.New()

	applicationCreateUseCase = usecase.NewCreateApplication(applicationRepo, v)
	applicationDeleteUseCase = usecase.NewDeleteApplication(applicationRepo)
	applicationUpdateUseCase = usecase.NewUpdateApplication(applicationRepo, v)

	categoryCreateUseCase = usecase.NewCreateUserCategory(dashboardRepo, categoryRepo, v)
	categoryUpdateUseCase = usecase.NewUpdateUserCategory(dashboardRepo, categoryRepo, v)
	categoryDeleteUseCase = usecase.NewDeleteUserCategory(dashboardRepo, categoryRepo)
	log.Println("Initialized category mutation use cases")

	bookmarkCreateUseCase = usecase.NewCreazeUserBookmark(dashboardRepo, categoryRepo, bookmarkRepo, v)
	bookmarkUpdateUseCase = usecase.NewUpdateUserBookmark(dashboardRepo, categoryRepo, bookmarkRepo, v)
	bookmarkDeleteUseCase = usecase.NewDeleteUserBookmark(dashboardRepo, categoryRepo, bookmarkRepo)
	log.Println("Initialized bookmark mutation use cases")

	engine := htmltpl.New("./views", ".html")
	app = fiber.New(fiber.Config{
		DisableStartupMessage: true,
		ReadBufferSize:        1024 * 1024 * 1,
		Views:                 engine,
	})
	app.Static("/static", "./static")
	log.Println("Server initialized")
}

func main() {
	interruptCtx, cancel := signal.NotifyContext(
		context.Background(),
		os.Interrupt,
		syscall.SIGTERM,
	)
	defer cancel()

	log.Println("---")

	endpoint.Session(app)
	log.Println("Registered session endpoints")

 endpoint.Dashboard(endpoint.DashboardDeps{
		App:                app,
		GetUserDashboard:   getUserDashboardUseCase,
		GetUserSettings:    getUserSettingsUseCase,
		EnsureDefaultTheme: ensureDefaultThemeUseCase,
		GetUserThemeByID:   getUserThemeByIDUseCase,
	})
	log.Println("Registered dashboard endpoints")

	endpoint.Application(endpoint.ApplicationDeps{
		App:                 app,
		CreateApplication:   applicationCreateUseCase,
		DeleteApplication:   applicationDeleteUseCase,
		UpdateApplication:   applicationUpdateUseCase,
		GetUserApplications: getUserApplicationsUseCase,
		ListApplications:    getApplicationsUseCase,
		GetApplication:      getApplicationUseCase,
	})
	log.Println("Registered application endpoints")

 endpoint.Category(endpoint.CategoryDeps{
		App:                      app,
		GetUserCategories:        getUserCategoriesUseCase,
		GetUserShelvedCategories: getUserShelvedCategoriesUseCase,
		GetUserCategory:          getUserCategoryUseCase,
		CategoryCreate:           categoryCreateUseCase,
		CategoryUpdate:           categoryUpdateUseCase,
		CategoryDelete:           categoryDeleteUseCase,
	})
	log.Println("Registered category endpoints")

	endpoint.Bookmark(endpoint.BookmarkDeps{
		App:               app,
		GetUserBookmarks:  getUserBookmarksUseCase,
		GetUserBookmark:   getUserBookmarkUseCase,
		GetUserCategories: getUserCategoriesUseCase,
		GetUserCategory:   getUserCategoryUseCase,
		BookmarkCreate:    bookmarkCreateUseCase,
		BookmarkUpdate:    bookmarkUpdateUseCase,
		BookmarkDelete:    bookmarkDeleteUseCase,
	})
	log.Println("Registered bookmark endpoints")

	endpoint.Setting(endpoint.SettingDeps{
		App:                app,
		GetUserSettings:    getUserSettingsUseCase,
		UpdateUserSettings: updateUserSettingsUseCase,
		ListUserThemes:     listUserThemesUseCase,
		EnsureDefaultTheme: ensureDefaultThemeUseCase,
	})
	log.Println("Registered setting endpoints")

	endpoint.Theme(endpoint.ThemeDeps{
		App:             app,
		ListUserThemes:  listUserThemesUseCase,
		CreateUserTheme: createUserThemeUseCase,
		DeleteUserTheme: deleteUserThemeUseCase,
		GetUserSettings: getUserSettingsUseCase,
	})
	log.Println("Registered theme endpoints")

	go func() {
		if err := app.Listen(":3000"); err != nil {
			log.Printf("Server error: %v\n", err)
		}
	}()
	log.Println("Server listening on port 3000")

	<-interruptCtx.Done()

	if err := app.Shutdown(); err != nil {
		log.Printf("Server shutdown error: %v\n", err)
	}
	log.Println("Server shutdown")

	log.Println("***")
}
