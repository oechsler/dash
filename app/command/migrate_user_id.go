package command

import (
	"context"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// UserIDMigrator migrates user-owned data from a legacy user ID to the stable sub claim.
//
// TODO(v3): remove together with MigrateUserID, UserIDMigrationRepository, and the
// migration block in the session handler once all deployments have gone through at
// least one login cycle after upgrading to the issuer-scoped UserID scheme.
type UserIDMigrator interface {
	Handle(ctx context.Context, oldUserID, newUserID string) error
}

// TODO(v3): remove together with UserIDMigrator.
type MigrateUserID struct {
	repo domainrepo.UserIDMigrationRepository
}

func NewMigrateUserID(repo domainrepo.UserIDMigrationRepository) *MigrateUserID {
	return &MigrateUserID{repo: repo}
}

func (h *MigrateUserID) Handle(ctx context.Context, oldUserID, newUserID string) error {
	if oldUserID == "" || oldUserID == newUserID {
		return nil
	}
	return h.repo.MigrateUserID(ctx, oldUserID, newUserID)
}
