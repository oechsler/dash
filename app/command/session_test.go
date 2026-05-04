package command_test

import (
	"context"
	"errors"
	"testing"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

// ── InvalidateSession ──────────────────────────────────────────────────────

func TestInvalidateSession_Handle_Success(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("DeleteByID", mock.Anything, "record-1", "user-1").Return(nil)

	h := command.NewInvalidateSession(repo)
	err := h.Handle(context.Background(), "user-1", "record-1")

	require.NoError(t, err)
	repo.AssertExpectations(t)
}

func TestInvalidateSession_Handle_RepoError(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("DeleteByID", mock.Anything, "record-1", "user-1").Return(errors.New("db error"))

	h := command.NewInvalidateSession(repo)
	err := h.Handle(context.Background(), "user-1", "record-1")

	require.Error(t, err)
}

// ── TerminateSession ───────────────────────────────────────────────────────

func TestTerminateSession_Handle_Success(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("DeleteBySessionID", mock.Anything, "session-abc").Return(nil)

	h := command.NewTerminateSession(repo)
	err := h.Handle(context.Background(), "session-abc")

	require.NoError(t, err)
	repo.AssertExpectations(t)
}

func TestTerminateSession_Handle_RepoError(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("DeleteBySessionID", mock.Anything, "session-abc").Return(errors.New("db error"))

	h := command.NewTerminateSession(repo)
	err := h.Handle(context.Background(), "session-abc")

	require.Error(t, err)
}

// ── CleanupSessions ────────────────────────────────────────────────────────

func TestCleanupSessions_Handle_Success(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("DeleteExpired", mock.Anything).Return(nil)

	h := command.NewCleanupSessions(repo)
	err := h.Handle(context.Background())

	require.NoError(t, err)
	repo.AssertExpectations(t)
}

func TestCleanupSessions_Handle_RepoError(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("DeleteExpired", mock.Anything).Return(errors.New("db error"))

	h := command.NewCleanupSessions(repo)
	err := h.Handle(context.Background())

	require.Error(t, err)
}

// ── PinSession ─────────────────────────────────────────────────────────────

func TestPinSession_Handle_Success(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("Pin", mock.Anything, "session-abc", "user-1", mock.AnythingOfType("time.Time")).Return(nil)

	h := command.NewPinSession(repo)
	err := h.Handle(context.Background(), "user-1", command.PinSessionCmd{SessionID: "session-abc"})

	require.NoError(t, err)
	repo.AssertExpectations(t)
}

func TestPinSession_Handle_RepoError(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("Pin", mock.Anything, mock.Anything, mock.Anything, mock.Anything).Return(errors.New("db error"))

	h := command.NewPinSession(repo)
	err := h.Handle(context.Background(), "user-1", command.PinSessionCmd{SessionID: "session-abc"})

	require.Error(t, err)
}

// ── UnpinSession ───────────────────────────────────────────────────────────

func TestUnpinSession_Handle_Success(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("Unpin", mock.Anything, "session-abc", "user-1").Return(nil)

	h := command.NewUnpinSession(repo)
	err := h.Handle(context.Background(), "user-1", "session-abc")

	require.NoError(t, err)
	repo.AssertExpectations(t)
}

func TestUnpinSession_Handle_RepoError(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("Unpin", mock.Anything, mock.Anything, mock.Anything).Return(errors.New("db error"))

	h := command.NewUnpinSession(repo)
	err := h.Handle(context.Background(), "user-1", "session-abc")

	require.Error(t, err)
}
