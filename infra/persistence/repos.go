package persistence

import (
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	"git.at.oechsler.it/samuel/dash/v2/infra/persistence/repo"

	"gorm.io/gorm"
)

type Repos struct {
	User            domainrepo.UserRepository
	Dashboard       domainrepo.DashboardRepository
	Category        domainrepo.CategoryRepository
	Bookmark        domainrepo.BookmarkRepository
	Application     domainrepo.ApplicationRepository
	Setting         domainrepo.SettingRepository
	Theme           domainrepo.ThemeRepository
	Session         domainrepo.SessionRepository
	UserIDMigration domainrepo.UserIDMigrationRepository
	IdpLink         domainrepo.IdpLinkRepository
}

func NewRepos(db *gorm.DB) (*Repos, error) {
	// userRepo must be initialised first: it creates the users table, runs the
	// backfill, and adds ON DELETE CASCADE FK constraints on all dependent
	// tables before the other repos run their own AutoMigrate calls.
	userRepo, err := repo.NewGormUserRepo(db)
	if err != nil {
		return nil, err
	}

	dashboardRepo, err := repo.NewGormDashboardRepo(db)
	if err != nil {
		return nil, err
	}

	categoryRepo, err := repo.NewGormCategoryRepo(db)
	if err != nil {
		return nil, err
	}

	bookmarkRepo, err := repo.NewGormBookmarkRepo(db)
	if err != nil {
		return nil, err
	}

	applicationRepo, err := repo.NewGormApplicationRepo(db)
	if err != nil {
		return nil, err
	}

	// themeRepo must be initialised before settingRepo: Setting.Theme carries
	// a constraint tag (fk_settings_theme, ON DELETE RESTRICT) that GORM
	// resolves during AutoMigrate, which requires the themes table to exist.
	themeRepo, err := repo.NewGormThemeRepo(db)
	if err != nil {
		return nil, err
	}

	settingRepo, err := repo.NewGormSettingRepo(db)
	if err != nil {
		return nil, err
	}

	sessionRepo, err := repo.NewGormSessionRepo(db)
	if err != nil {
		return nil, err
	}

	idpLinkRepo, err := repo.NewGormIdpLinkRepo(db, userRepo)
	if err != nil {
		return nil, err
	}

	return &Repos{
		User:            userRepo,
		Dashboard:       dashboardRepo,
		Category:        categoryRepo,
		Bookmark:        bookmarkRepo,
		Application:     applicationRepo,
		Setting:         settingRepo,
		Theme:           themeRepo,
		Session:         sessionRepo,
		UserIDMigration: repo.NewGormUserIDMigrationRepo(db),
		IdpLink:         idpLinkRepo,
	}, nil
}
