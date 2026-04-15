package command

import (
	"context"
	"time"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// PinSessionCmd carries the data needed to pin the current session.
type PinSessionCmd struct {
	SessionID string
}

// SessionPinner handles the pin-session command.
type SessionPinner interface {
	Handle(ctx context.Context, userID string, cmd PinSessionCmd) error
}

type PinSession struct {
	Repo domainrepo.SessionRepository
}

func NewPinSession(repo domainrepo.SessionRepository) *PinSession {
	return &PinSession{Repo: repo}
}

const pinnedSessionWindow = 365 * 24 * time.Hour

func (h *PinSession) Handle(ctx context.Context, userID string, cmd PinSessionCmd) error {
	return h.Repo.Pin(ctx, cmd.SessionID, userID, time.Now().Add(pinnedSessionWindow))
}
