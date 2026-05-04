package command_test

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

func TestCreateSession_Handle_Success(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("Create", mock.Anything, mock.MatchedBy(func(r interface{}) bool {
		return r != nil
	})).Return(nil)

	cmd := command.CreateSessionCmd{
		SessionID:   "cookie-session-1",
		UserID:      "user-1",
		Sub:         "sub123",
		Username:    "sam",
		Email:       "sam@example.com",
		FirstName:   "Sam",
		LastName:    "O",
		DisplayName: "Sam O",
		Groups:      []string{"dev"},
		IsAdmin:     false,
		IssuedAt:    time.Now(),
		ExpiresAt:   time.Now().Add(time.Hour),
		IP:          "127.0.0.1",
		UserAgent:   "Mozilla/5.0",
	}

	h := command.NewCreateSession(repo)
	err := h.Handle(context.Background(), cmd)

	require.NoError(t, err)
	repo.AssertExpectations(t)
}

func TestCreateSession_Handle_RepoError(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("Create", mock.Anything, mock.Anything).Return(errors.New("db error"))

	h := command.NewCreateSession(repo)
	err := h.Handle(context.Background(), command.CreateSessionCmd{})

	require.Error(t, err)
}
