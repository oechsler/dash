package persistence

import (
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	"git.at.oechsler.it/samuel/dash/v2/infra/persistence/repo"

	"gorm.io/gorm"
)

type Repos struct {
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

func NewRepos(db *gorm.DB) (*Repos, error) {
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

	settingRepo, err := repo.NewGormSettingRepo(db)
	if err != nil {
		return nil, err
	}

	themeRepo, err := repo.NewGormThemeRepo(db)
	if err != nil {
		return nil, err
	}

	sessionRepo, err := repo.NewGormSessionRepo(db)
	if err != nil {
		return nil, err
	}

	idpLinkRepo, err := repo.NewGormIdpLinkRepo(db)
	if err != nil {
		return nil, err
	}

	return &Repos{
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
