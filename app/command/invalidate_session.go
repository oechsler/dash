package command

import (
	"context"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// SessionInvalidator handles the invalidate-session command.
// Invalidation deletes the record entirely, causing the revocation check in
// LoadIdentity to deny access on the next request from that device.
type SessionInvalidator interface {
	Handle(ctx context.Context, userID string, sessionID string) error
}

type InvalidateSession struct {
	Repo domainrepo.SessionRepository
}

func NewInvalidateSession(repo domainrepo.SessionRepository) *InvalidateSession {
	return &InvalidateSession{Repo: repo}
}

func (h *InvalidateSession) Handle(ctx context.Context, userID string, sessionID string) error {
	return h.Repo.DeleteByID(ctx, sessionID, userID)
}
