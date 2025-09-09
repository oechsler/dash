package main

import (
	"context"
	"dash/data/repo"
	"dash/domain/usecase"
	"dash/domain/validation"
	"dash/endpoint"
	"dash/environment"
	"log"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/gofiber/fiber/v2"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

var (
	env                             *environment.Env
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
	getAvailableIconTypesUseCase    *usecase.GetAvailableIconTypes
)

func init() {
	env = environment.NewEnv()

	var err error
	dbPath := env.String("DB_PATH", "./dash.db")
	log.Printf("connecting to database: %s", dbPath)
	db, err = gorm.Open(sqlite.Open(dbPath), &gorm.Config{
		PrepareStmt: true,
	})
	if err != nil {
		log.Fatalf("failed to connect database: %v", err)
	}

	dashboardRepo, err = repo.NewGormDashboardRepo(db)
	if err != nil {
		log.Fatal("failed to initialize dashboard repo")
	}

	categoryRepo, err = repo.NewGormCategoryRepo(db)
	if err != nil {
		log.Fatal("failed to initialize category repo")
	}

	bookmarkRepo, err = repo.NewGormBookmarkRepo(db)
	if err != nil {
		log.Fatal("failed to initialize bookmark repo")
	}

	applicationRepo, err = repo.NewGormApplicationRepo(db)
	if err != nil {
		log.Fatal("failed to initialize application repo")
	}

	settingRepo, err = repo.NewGormSettingRepo(db)
	if err != nil {
		log.Fatal("failed to initialize setting repo")
	}

	themeRepo, err = repo.NewGormThemeRepo(db)
	if err != nil {
		log.Fatal("failed to initialize theme repo")
	}

	getApplicationsUseCase = usecase.NewListApplications(applicationRepo)

	getUserApplicationsUseCase = usecase.NewGetUserApplications(getApplicationsUseCase)

	getApplicationUseCase = usecase.NewGetApplication(applicationRepo)

	getUserBookmarkUseCase = usecase.NewGetUserBookmark(dashboardRepo, bookmarkRepo, categoryRepo)

	getUserCategoriesUseCase = usecase.NewGetUserCategories(dashboardRepo, categoryRepo, bookmarkRepo)

	getUserCategoryUseCase = usecase.NewGetUserCategory(dashboardRepo, categoryRepo)

	getUserShelvedCategoriesUseCase = usecase.NewGetUserShelvedCategories(dashboardRepo, categoryRepo, bookmarkRepo)

	getUserDashboardGreetingUseCase = usecase.NewGetUserDashboardGreeting()

	getUserDashboardUseCase = usecase.NewGetUserDashboard(dashboardRepo, getUserCategoriesUseCase, getUserApplicationsUseCase, getUserDashboardGreetingUseCase)

	v := validation.New()

	ensureDefaultThemeUseCase = usecase.NewEnsureDefaultTheme(themeRepo)
	getUserSettingsUseCase = usecase.NewGetUserSettings(settingRepo, themeRepo, ensureDefaultThemeUseCase)
	updateUserSettingsUseCase = usecase.NewUpdateUserSettings(settingRepo, themeRepo, v)
	listUserThemesUseCase = usecase.NewListUserThemes(themeRepo, settingRepo)
	createUserThemeUseCase = usecase.NewCreateUserTheme(themeRepo, v)
	deleteUserThemeUseCase = usecase.NewDeleteUserTheme(themeRepo)
	getUserThemeByIDUseCase = usecase.NewGetUserThemeByID(themeRepo)

	applicationCreateUseCase = usecase.NewCreateApplication(applicationRepo, v)
	applicationDeleteUseCase = usecase.NewDeleteApplication(applicationRepo)
	applicationUpdateUseCase = usecase.NewUpdateApplication(applicationRepo, v)

	categoryCreateUseCase = usecase.NewCreateUserCategory(dashboardRepo, categoryRepo, v)
	categoryUpdateUseCase = usecase.NewUpdateUserCategory(dashboardRepo, categoryRepo, v)
	categoryDeleteUseCase = usecase.NewDeleteUserCategory(dashboardRepo, categoryRepo)

	bookmarkCreateUseCase = usecase.NewCreazeUserBookmark(dashboardRepo, categoryRepo, bookmarkRepo, v)
	bookmarkUpdateUseCase = usecase.NewUpdateUserBookmark(dashboardRepo, categoryRepo, bookmarkRepo, v)
	bookmarkDeleteUseCase = usecase.NewDeleteUserBookmark(dashboardRepo, categoryRepo, bookmarkRepo)

	getAvailableIconTypesUseCase = usecase.NewGetAvailableIconTypes()

	app = fiber.New(fiber.Config{
		DisableStartupMessage: true,
		ReadBufferSize:        1024 * 1024 * 1,
	})
	app.Use(func(c *fiber.Ctx) error {
		start := time.Now()
		err := c.Next()

		ip := c.IP()
		latency := time.Since(start)
		status := c.Response().StatusCode()
		method := c.Method()
		path := c.Path()
		userAgent := c.Get("User-Agent")

		log.Printf("[%d] %s %s - %v - %s - %s",
			status,
			method,
			path,
			latency,
			ip,
			userAgent,
		)

		return err
	})
	app.Static("/static", "./static")
}

func main() {
	interruptCtx, cancel := signal.NotifyContext(
		context.Background(),
		os.Interrupt,
		syscall.SIGTERM,
	)
	defer cancel()

	endpoint.Session(env, app)
	endpoint.Favicon(env, app)

	endpoint.Dashboard(endpoint.DashboardDeps{
		Env:                env,
		App:                app,
		GetUserDashboard:   getUserDashboardUseCase,
		GetUserSettings:    getUserSettingsUseCase,
		EnsureDefaultTheme: ensureDefaultThemeUseCase,
		GetUserThemeByID:   getUserThemeByIDUseCase,
	})

	endpoint.Application(endpoint.ApplicationDeps{
		Env:                   env,
		App:                   app,
		CreateApplication:     applicationCreateUseCase,
		DeleteApplication:     applicationDeleteUseCase,
		UpdateApplication:     applicationUpdateUseCase,
		GetUserApplications:   getUserApplicationsUseCase,
		ListApplications:      getApplicationsUseCase,
		GetApplication:        getApplicationUseCase,
		GetAvailableIconTypes: getAvailableIconTypesUseCase,
	})

	endpoint.Category(endpoint.CategoryDeps{
		Env:                      env,
		App:                      app,
		GetUserCategories:        getUserCategoriesUseCase,
		GetUserShelvedCategories: getUserShelvedCategoriesUseCase,
		GetUserCategory:          getUserCategoryUseCase,
		CategoryCreate:           categoryCreateUseCase,
		CategoryUpdate:           categoryUpdateUseCase,
		CategoryDelete:           categoryDeleteUseCase,
	})

	endpoint.Bookmark(endpoint.BookmarkDeps{
		Env:                      env,
		App:                      app,
		GetUserBookmark:          getUserBookmarkUseCase,
		GetUserCategory:          getUserCategoryUseCase,
		GetUserCategories:        getUserCategoriesUseCase,
		GetUserShelvedCategories: getUserShelvedCategoriesUseCase,
		BookmarkCreate:           bookmarkCreateUseCase,
		BookmarkUpdate:           bookmarkUpdateUseCase,
		BookmarkDelete:           bookmarkDeleteUseCase,
		GetAvailableIconTypes:    getAvailableIconTypesUseCase,
	})

	endpoint.Setting(endpoint.SettingDeps{
		Env:                env,
		App:                app,
		GetUserSettings:    getUserSettingsUseCase,
		UpdateUserSettings: updateUserSettingsUseCase,
		ListUserThemes:     listUserThemesUseCase,
		EnsureDefaultTheme: ensureDefaultThemeUseCase,
	})

	endpoint.Theme(endpoint.ThemeDeps{
		Env:             env,
		App:             app,
		ListUserThemes:  listUserThemesUseCase,
		CreateUserTheme: createUserThemeUseCase,
		DeleteUserTheme: deleteUserThemeUseCase,
		GetUserSettings: getUserSettingsUseCase,
	})

	go func() {
		if err := app.Listen(":3000"); err != nil {
			log.Printf("server error: %v\n", err)
		}
	}()
	log.Println("server listening on 0.0.0.0:3000")

	<-interruptCtx.Done()

	if err := app.Shutdown(); err != nil {
		log.Printf("server shutdown error: %v\n", err)
	}
}
