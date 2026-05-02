package command

import (
	"context"
	"time"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	"github.com/google/uuid"
)

// CreateSessionCmd carries the data needed to persist a new session on login.
// Identity fields are extracted from the OIDC token at login time and stored
// server-side so the cookie only needs to carry the SessionID.
type CreateSessionCmd struct {
	SessionID   string
	UserID      string
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
	IP          string
	UserAgent   string
}

// SessionCreator handles the create-session command.
type SessionCreator interface {
	Handle(ctx context.Context, cmd CreateSessionCmd) error
}

type CreateSession struct {
	Repo domainrepo.SessionRepository
}

func NewCreateSession(repo domainrepo.SessionRepository) *CreateSession {
	return &CreateSession{Repo: repo}
}

func (h *CreateSession) Handle(ctx context.Context, cmd CreateSessionCmd) error {
	return h.Repo.Create(ctx, &domainrepo.SessionRecord{
		ID:          uuid.New().String(),
		UserID:      cmd.UserID,
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
		LastIP:      cmd.IP,
		UserAgent:   cmd.UserAgent,
	})
}
