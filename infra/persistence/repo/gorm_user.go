package repo

import (
	"context"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	"git.at.oechsler.it/samuel/dash/v2/infra/persistence/model"

	"gorm.io/gorm"
)

var _ domainrepo.UserRepository = (*GormUserRepo)(nil)

type GormUserRepo struct {
	db *gorm.DB
}

// NewGormUserRepo creates the users table and backfills it from existing data.
// It must be initialised before all other repos: the FK constraints declared
// via GORM constraint tags (e.g. `gorm:"constraint:fk_dashboards_user,OnDelete:CASCADE"`)
// are created by each repo's own AutoMigrate call, which requires the users
// table to already exist and be fully populated.
//
// TODO(v3): remove the backfill block once all deployments have run at least
// one startup after upgrading to this version.
func NewGormUserRepo(db *gorm.DB) (*GormUserRepo, error) {
	if err := db.AutoMigrate(&model.User{}); err != nil {
		return nil, err
	}

	// Backfill: insert every user_id that already exists in dependent tables
	// so that the FK constraints added by subsequent AutoMigrate calls do not
	// fail due to missing parent rows.
	//
	// TODO(v3): drop this block — by that point all deployments will have
	// gone through at least one startup that populated users via
	// IdpLinkRepository.ResolveOrCreate.
	backfillSQL := `
		INSERT INTO users (id)
		SELECT DISTINCT user_id FROM dashboards    WHERE user_id IS NOT NULL AND user_id != ''
		UNION
		SELECT DISTINCT user_id FROM settings      WHERE user_id IS NOT NULL AND user_id != ''
		UNION
		SELECT DISTINCT user_id FROM themes        WHERE user_id IS NOT NULL AND user_id != ''
		UNION
		SELECT DISTINCT user_id FROM sessions      WHERE user_id IS NOT NULL AND user_id != ''
		UNION
		SELECT DISTINCT user_id FROM idp_links       WHERE user_id IS NOT NULL AND user_id != ''
		ON CONFLICT DO NOTHING
	`
	if err := db.Exec(backfillSQL).Error; err != nil {
		return nil, err
	}

	return &GormUserRepo{db: db}, nil
}

// EnsureExists inserts a user row if one does not already exist.
// Called by IdpLinkRepository.ResolveOrCreate before creating an idp_link.
func (r *GormUserRepo) EnsureExists(ctx context.Context, id string) error {
	return r.db.WithContext(ctx).
		Exec("INSERT INTO users (id) VALUES (?) ON CONFLICT DO NOTHING", id).
		Error
}

func (r *GormUserRepo) DeleteByID(ctx context.Context, id string) error {
	return r.db.WithContext(ctx).
		Where("id = ?", id).
		Delete(&model.User{}).
		Error
}
