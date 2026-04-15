package command

import (
	"context"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// SessionTerminator handles voluntary logout: deletes the session record by
// cookie SessionID so it no longer appears in the session overview.
type SessionTerminator interface {
	Handle(ctx context.Context, sessionID string) error
}

type TerminateSession struct {
	Repo domainrepo.SessionRepository
}

func NewTerminateSession(repo domainrepo.SessionRepository) *TerminateSession {
	return &TerminateSession{Repo: repo}
}

func (h *TerminateSession) Handle(ctx context.Context, sessionID string) error {
	return h.Repo.DeleteBySessionID(ctx, sessionID)
}
