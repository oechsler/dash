package repo

import (
	"context"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	"gorm.io/gorm"
)

var _ domainrepo.UserIDMigrationRepository = (*GormUserIDMigrationRepo)(nil)

type GormUserIDMigrationRepo struct {
	db *gorm.DB
}

func NewGormUserIDMigrationRepo(db *gorm.DB) *GormUserIDMigrationRepo {
	return &GormUserIDMigrationRepo{db: db}
}

// MigrateUserID updates user_id from oldID to newID across all user-owned tables
// in a single transaction. Called on login when the OIDC sub claim differs from
// the previously used preferred_username / name fallback.
func (r *GormUserIDMigrationRepo) MigrateUserID(ctx context.Context, oldID, newID string) error {
	if oldID == newID {
		return nil
	}
	return r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		for _, table := range []string{"dashboards", "settings", "themes", "sessions"} {
			if err := tx.Exec(
				"UPDATE "+table+" SET user_id = ? WHERE user_id = ?",
				newID, oldID,
			).Error; err != nil {
				return err
			}
		}
		return nil
	})
}
