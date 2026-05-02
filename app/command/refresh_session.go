package command

import (
	"context"
	"time"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// RefreshSessionCmd carries updated token timing and identity for an existing
// session after re-authentication via OIDC. Identity fields are re-extracted
// from the new token so group changes take effect immediately.
type RefreshSessionCmd struct {
	SessionID   string
	Sub         string
	Username    string
	Email       string
	FirstName   string
	LastName    string
	DisplayName string
	Picture     string
	ProfileUrl  string
	Groups      []string
	IsAdmin     bool
	IssuedAt    time.Time
	ExpiresAt   time.Time
}

// SessionRefresher handles the refresh-session command.
type SessionRefresher interface {
	Handle(ctx context.Context, cmd RefreshSessionCmd) error
}

type RefreshSession struct {
	repo domainrepo.SessionRepository
}

func NewRefreshSession(repo domainrepo.SessionRepository) *RefreshSession {
	return &RefreshSession{repo: repo}
}

func (h *RefreshSession) Handle(ctx context.Context, cmd RefreshSessionCmd) error {
	return h.repo.RefreshBySessionID(ctx, &domainrepo.SessionRecord{
		SessionID:   cmd.SessionID,
		Sub:         cmd.Sub,
		Username:    cmd.Username,
		Email:       cmd.Email,
		FirstName:   cmd.FirstName,
		LastName:    cmd.LastName,
		DisplayName: cmd.DisplayName,
		Picture:     cmd.Picture,
		ProfileUrl:  cmd.ProfileUrl,
		Groups:      cmd.Groups,
		IsAdmin:     cmd.IsAdmin,
		IssuedAt:    cmd.IssuedAt,
		ExpiresAt:   cmd.ExpiresAt,
	})
}
