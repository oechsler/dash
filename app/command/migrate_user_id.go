package command

import (
	"context"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// UserIDMigrator migrates user-owned data from a legacy user ID to the stable sub claim.
type UserIDMigrator interface {
	Handle(ctx context.Context, oldUserID, newUserID string) error
}

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
