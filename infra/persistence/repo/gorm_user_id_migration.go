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
// After migrating all rows the legacy users entry for oldID is removed; it was
// only inserted during the backfill step and is no longer needed once all data
// has been moved to newID.
//
// TODO(v3): remove together with LegacyUserID and the backfill block in
// NewGormUserRepo once all deployments have completed the migration.
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
		// Remove the legacy users row; CASCADE will clean up any remaining
		// references (there should be none after the UPDATE loop above).
		if err := tx.Exec("DELETE FROM users WHERE id = ?", oldID).Error; err != nil {
			return err
		}
		return nil
	})
}
