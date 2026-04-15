package command

import (
	"context"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// SessionCleaner handles the cleanup-sessions command.
type SessionCleaner interface {
	Handle(ctx context.Context) error
}

type CleanupSessions struct {
	Repo domainrepo.SessionRepository
}

func NewCleanupSessions(repo domainrepo.SessionRepository) *CleanupSessions {
	return &CleanupSessions{Repo: repo}
}

// Handle deletes all sessions whose OIDC token has expired and that are no
// longer pinned (or whose pin has also expired).
func (h *CleanupSessions) Handle(ctx context.Context) error {
	return h.Repo.DeleteExpired(ctx)
}
