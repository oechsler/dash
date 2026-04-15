package command

import (
	"context"
	"time"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	"github.com/google/uuid"
)

// CreateSessionCmd carries the data needed to persist a new session on login.
type CreateSessionCmd struct {
	SessionID   string
	UserID      string
	IssuedAt    time.Time
	ExpiresAt   time.Time
	IP          string
	UserAgent   string
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
		ID:        uuid.New().String(),
		UserID:    cmd.UserID,
		SessionID: cmd.SessionID,
		IssuedAt:  cmd.IssuedAt,
		ExpiresAt: cmd.ExpiresAt,
		LastIP:    cmd.IP,
		UserAgent: cmd.UserAgent,
		Sub:       cmd.Sub,
		Username:  cmd.Username,
		Email:     cmd.Email,
		FirstName: cmd.FirstName,
		LastName:  cmd.LastName,
		DisplayName: cmd.DisplayName,
		Picture:   cmd.Picture,
		ProfileUrl: cmd.ProfileUrl,
		Groups:    cmd.Groups,
		IsAdmin:   cmd.IsAdmin,
	})
}
