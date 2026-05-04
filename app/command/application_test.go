package command_test

import (
	"context"
	"errors"
	"testing"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

// ── CreateApplication ──────────────────────────────────────────────────────

func TestCreateApplication_Handle_ValidationError(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(errors.New("failed"))

	h := command.NewCreateApplication(nil, v)
	err := h.Handle(context.Background(), command.CreateApplicationCmd{})

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestCreateApplication_Handle_InvalidIcon(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	h := command.NewCreateApplication(nil, v)
	err := h.Handle(context.Background(), command.CreateApplicationCmd{
		Icon:        "bad-icon",
		DisplayName: "App",
		Url:         "https://example.com",
	})

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestCreateApplication_Handle_Success(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.ApplicationRecord) bool {
		return r.DisplayName == "My App" && r.Url == "https://example.com"
	})).Return(nil)

	h := command.NewCreateApplication(appRepo, v)
	err := h.Handle(context.Background(), command.CreateApplicationCmd{
		Icon:        "mdi:home",
		DisplayName: "My App",
		Url:         "https://example.com",
	})

	require.NoError(t, err)
	appRepo.AssertExpectations(t)
}

func TestCreateApplication_Handle_RepoError(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("Upsert", mock.Anything, mock.Anything).Return(errors.New("db error"))

	h := command.NewCreateApplication(appRepo, v)
	err := h.Handle(context.Background(), command.CreateApplicationCmd{
		Icon:        "mdi:home",
		DisplayName: "My App",
		Url:         "https://example.com",
	})

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

// ── UpdateApplication ──────────────────────────────────────────────────────

func TestUpdateApplication_Handle_ValidationError(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(errors.New("failed"))

	h := command.NewUpdateApplication(nil, v)
	err := h.Handle(context.Background(), command.UpdateApplicationCmd{})

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestUpdateApplication_Handle_InvalidIcon(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	h := command.NewUpdateApplication(nil, v)
	err := h.Handle(context.Background(), command.UpdateApplicationCmd{
		ID:   1,
		Icon: "bad",
		Url:  "https://example.com",
	})

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestUpdateApplication_Handle_NotFound(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("Get", mock.Anything, uint(1)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityApplication))

	h := command.NewUpdateApplication(appRepo, v)
	err := h.Handle(context.Background(), command.UpdateApplicationCmd{
		ID:   1,
		Icon: "mdi:home",
		Url:  "https://example.com",
	})

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestUpdateApplication_Handle_Success(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	existing := &domainrepo.ApplicationRecord{ID: 1, Icon: "mdi:old", DisplayName: "Old"}
	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("Get", mock.Anything, uint(1)).Return(existing, nil)
	appRepo.On("Upsert", mock.Anything, mock.MatchedBy(func(r *domainrepo.ApplicationRecord) bool {
		return r.DisplayName == "New App"
	})).Return(nil)

	h := command.NewUpdateApplication(appRepo, v)
	err := h.Handle(context.Background(), command.UpdateApplicationCmd{
		ID:          1,
		Icon:        "mdi:home",
		DisplayName: "New App",
		Url:         "https://example.com",
	})

	require.NoError(t, err)
}

// ── DeleteApplication ──────────────────────────────────────────────────────

func TestDeleteApplication_Handle_ZeroID(t *testing.T) {
	h := command.NewDeleteApplication(nil)
	err := h.Handle(context.Background(), 0)

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestDeleteApplication_Handle_NotFound(t *testing.T) {
	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("Get", mock.Anything, uint(5)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityApplication))

	h := command.NewDeleteApplication(appRepo)
	err := h.Handle(context.Background(), 5)

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestDeleteApplication_Handle_Success(t *testing.T) {
	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.ApplicationRecord{ID: 5}, nil)
	appRepo.On("Delete", mock.Anything, uint(5)).Return(nil)

	h := command.NewDeleteApplication(appRepo)
	err := h.Handle(context.Background(), 5)

	require.NoError(t, err)
	appRepo.AssertExpectations(t)
}

func TestDeleteApplication_Handle_DeleteError(t *testing.T) {
	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("Get", mock.Anything, uint(5)).
		Return(&domainrepo.ApplicationRecord{ID: 5}, nil)
	appRepo.On("Delete", mock.Anything, uint(5)).Return(errors.New("db error"))

	h := command.NewDeleteApplication(appRepo)
	err := h.Handle(context.Background(), 5)

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}
