package command

import (
	"context"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// SessionUnpinner handles the unpin-session command.
type SessionUnpinner interface {
	Handle(ctx context.Context, userID string, sessionID string) error
}

type UnpinSession struct {
	Repo domainrepo.SessionRepository
}

func NewUnpinSession(repo domainrepo.SessionRepository) *UnpinSession {
	return &UnpinSession{Repo: repo}
}

// Handle clears PinnedUntil on the session record, keeping it alive for as long
// as the OIDC token is valid. The record is NOT deleted — that would log the user
// out on the next request when the revocation check fails to find it.
func (h *UnpinSession) Handle(ctx context.Context, userID string, sessionID string) error {
	return h.Repo.Unpin(ctx, sessionID, userID)
}
