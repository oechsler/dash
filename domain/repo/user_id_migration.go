package repo

import "context"

// UserIDMigrationRepository handles the one-time per-user migration of all
// user-owned records from a legacy string-based user ID (preferred_username /
// name / email) to the stable OIDC sub claim.
//
// TODO(v3): remove once all deployments have gone through at least one login
// cycle after upgrading to the issuer-scoped UserID scheme.
type UserIDMigrationRepository interface {
	// MigrateUserID reassigns all user-owned records from oldID to newID.
	// No-op when oldID == newID or no records exist under oldID.
	MigrateUserID(ctx context.Context, oldID, newID string) error
}
