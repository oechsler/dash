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

// NewGormUserRepo creates the users table and wires up ON DELETE CASCADE
// foreign keys on all user-owned tables. Must be initialised before all
// other repos so that the FK constraints are in place when they run their
// own AutoMigrate calls.
//
// On first run against a pre-existing database the backfill step inserts
// all distinct user_id values found in dependent tables into users so that
// the subsequent ALTER TABLE … ADD CONSTRAINT statements succeed.
//
// TODO(v3): remove the backfill block once all deployments have run at
// least one startup after upgrading to this version.
func NewGormUserRepo(db *gorm.DB) (*GormUserRepo, error) {
	if err := db.AutoMigrate(&model.User{}); err != nil {
		return nil, err
	}

	// Backfill: collect every user_id that exists in dependent tables and
	// ensure each has a corresponding users row before we add FK constraints.
	//
	// TODO(v3): drop this block — by that point all deployments will have
	// gone through at least one startup that populated the users table via
	// IdpLinkRepository.ResolveOrCreate.
	backfillSQL := `
		INSERT INTO users (id)
		SELECT DISTINCT user_id FROM dashboards WHERE user_id IS NOT NULL AND user_id != ''
		UNION
		SELECT DISTINCT user_id FROM settings   WHERE user_id IS NOT NULL AND user_id != ''
		UNION
		SELECT DISTINCT user_id FROM themes      WHERE user_id IS NOT NULL AND user_id != ''
		UNION
		SELECT DISTINCT user_id FROM sessions    WHERE user_id IS NOT NULL AND user_id != ''
		UNION
		SELECT DISTINCT user_id FROM user_idp_links WHERE user_id IS NOT NULL AND user_id != ''
		ON CONFLICT DO NOTHING
	`
	if err := db.Exec(backfillSQL).Error; err != nil {
		return nil, err
	}

	// Add ON DELETE CASCADE foreign keys idempotently. The DO … EXCEPTION
	// block is a standard Postgres pattern for "add constraint if not yet
	// present" because ALTER TABLE ADD CONSTRAINT IF NOT EXISTS is not
	// supported for foreign keys.
	constraints := []struct {
		table      string
		constraint string
		column     string
	}{
		{"dashboards",    "fk_dashboards_user",    "user_id"},
		{"settings",      "fk_settings_user",      "user_id"},
		{"themes",        "fk_themes_user",         "user_id"},
		{"sessions",      "fk_sessions_user",       "user_id"},
		{"user_idp_links","fk_idp_links_user",      "user_id"},
	}
	for _, c := range constraints {
		sql := `DO $$ BEGIN
			ALTER TABLE ` + c.table + ` ADD CONSTRAINT ` + c.constraint + `
				FOREIGN KEY (` + c.column + `) REFERENCES users(id) ON DELETE CASCADE;
		EXCEPTION WHEN duplicate_object THEN NULL;
		END $$`
		if err := db.Exec(sql).Error; err != nil {
			return nil, err
		}
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
