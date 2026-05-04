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

func TestRefreshSession_Handle_Success(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("RefreshBySessionID", mock.Anything, mock.MatchedBy(func(r interface{}) bool {
		return r != nil
	})).Return(nil)

	cmd := command.RefreshSessionCmd{
		SessionID:   "session-abc",
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
	}

	h := command.NewRefreshSession(repo)
	err := h.Handle(context.Background(), cmd)

	require.NoError(t, err)
	repo.AssertExpectations(t)
}

func TestRefreshSession_Handle_RepoError(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("RefreshBySessionID", mock.Anything, mock.Anything).Return(errors.New("db error"))

	h := command.NewRefreshSession(repo)
	err := h.Handle(context.Background(), command.RefreshSessionCmd{})

	require.Error(t, err)
}
