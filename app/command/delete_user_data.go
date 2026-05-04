package command

import (
	"context"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// UserDataDeleter handles the delete-user-data command.
type UserDataDeleter interface {
	Handle(ctx context.Context, userID string) error
}

type DeleteUserData struct {
	UserRepo domainrepo.UserRepository
}

func NewDeleteUserData(userRepo domainrepo.UserRepository) *DeleteUserData {
	return &DeleteUserData{UserRepo: userRepo}
}

func (h *DeleteUserData) Handle(ctx context.Context, userID string) error {
	// Deleting the users row cascades to all dependent tables via FK constraints:
	// dashboards (→ categories → bookmarks), settings, themes, sessions, idp_links.
	if err := h.UserRepo.DeleteByID(ctx, userID); err != nil {
		return domainerrors.Internal("delete user data", err)
	}
	return nil
}
