package command_test

import (
	"context"
	"errors"
	"testing"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

// ── DeleteUserData ─────────────────────────────────────────────────────────

func TestDeleteUserData_Handle_Success(t *testing.T) {
	userRepo := &repoMock.UserRepository{}
	userRepo.On("DeleteByID", mock.Anything, "user-1").Return(nil)

	h := command.NewDeleteUserData(userRepo)
	err := h.Handle(context.Background(), "user-1")

	require.NoError(t, err)
	userRepo.AssertExpectations(t)
}

func TestDeleteUserData_Handle_RepoError(t *testing.T) {
	userRepo := &repoMock.UserRepository{}
	userRepo.On("DeleteByID", mock.Anything, "user-1").Return(errors.New("db error"))

	h := command.NewDeleteUserData(userRepo)
	err := h.Handle(context.Background(), "user-1")

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

// ── ResolveOrCreateUser ────────────────────────────────────────────────────

func TestResolveOrCreateUser_Handle_ExistingUser(t *testing.T) {
	idpRepo := &repoMock.IdpLinkRepository{}
	idpRepo.On("ResolveOrCreate", mock.Anything, "https://issuer.example.com", "sub123").
		Return("user-uuid-1", false, nil)

	h := command.NewResolveOrCreateUser(idpRepo)
	userID, isNew, err := h.Handle(context.Background(), "https://issuer.example.com", "sub123")

	require.NoError(t, err)
	require.Equal(t, "user-uuid-1", userID)
	require.False(t, isNew)
}

func TestResolveOrCreateUser_Handle_NewUser(t *testing.T) {
	idpRepo := &repoMock.IdpLinkRepository{}
	idpRepo.On("ResolveOrCreate", mock.Anything, "https://issuer.example.com", "sub456").
		Return("user-uuid-new", true, nil)

	h := command.NewResolveOrCreateUser(idpRepo)
	userID, isNew, err := h.Handle(context.Background(), "https://issuer.example.com", "sub456")

	require.NoError(t, err)
	require.Equal(t, "user-uuid-new", userID)
	require.True(t, isNew)
}

func TestResolveOrCreateUser_Handle_RepoError(t *testing.T) {
	idpRepo := &repoMock.IdpLinkRepository{}
	idpRepo.On("ResolveOrCreate", mock.Anything, mock.Anything, mock.Anything).
		Return("", false, errors.New("db error"))

	h := command.NewResolveOrCreateUser(idpRepo)
	_, _, err := h.Handle(context.Background(), "issuer", "sub")

	require.Error(t, err)
}

// ── MigrateUserID ──────────────────────────────────────────────────────────

func TestMigrateUserID_Handle_EmptyOldID_NoOp(t *testing.T) {
	migRepo := &repoMock.UserIDMigrationRepository{}

	h := command.NewMigrateUserID(migRepo)
	err := h.Handle(context.Background(), "", "new-user-1")

	require.NoError(t, err)
	migRepo.AssertNotCalled(t, "MigrateUserID")
}

func TestMigrateUserID_Handle_SameID_NoOp(t *testing.T) {
	migRepo := &repoMock.UserIDMigrationRepository{}

	h := command.NewMigrateUserID(migRepo)
	err := h.Handle(context.Background(), "same-id", "same-id")

	require.NoError(t, err)
	migRepo.AssertNotCalled(t, "MigrateUserID")
}

func TestMigrateUserID_Handle_DifferentIDs_Delegates(t *testing.T) {
	migRepo := &repoMock.UserIDMigrationRepository{}
	migRepo.On("MigrateUserID", mock.Anything, "old-id", "new-id").Return(nil)

	h := command.NewMigrateUserID(migRepo)
	err := h.Handle(context.Background(), "old-id", "new-id")

	require.NoError(t, err)
	migRepo.AssertExpectations(t)
}

func TestMigrateUserID_Handle_RepoError(t *testing.T) {
	migRepo := &repoMock.UserIDMigrationRepository{}
	migRepo.On("MigrateUserID", mock.Anything, "old-id", "new-id").Return(errors.New("db error"))

	h := command.NewMigrateUserID(migRepo)
	err := h.Handle(context.Background(), "old-id", "new-id")

	require.Error(t, err)
}
