package query_test

import (
	"context"
	"errors"
	"testing"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/query"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

// ── ListApplications ───────────────────────────────────────────────────────

func TestListApplications_Handle_RepoError(t *testing.T) {
	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("List", mock.Anything).Return(nil, errors.New("db error"))

	h := query.NewListApplications(appRepo)
	_, err := h.Handle(context.Background())

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

func TestListApplications_Handle_Empty(t *testing.T) {
	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("List", mock.Anything).Return([]domainrepo.ApplicationRecord{}, nil)

	h := query.NewListApplications(appRepo)
	apps, err := h.Handle(context.Background())

	require.NoError(t, err)
	require.Empty(t, apps)
}

func TestListApplications_Handle_Success(t *testing.T) {
	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("List", mock.Anything).Return([]domainrepo.ApplicationRecord{
		{ID: 1, Icon: "mdi:home", DisplayName: "Home", Url: "https://example.com", VisibleToGroups: []string{}},
	}, nil)

	h := query.NewListApplications(appRepo)
	apps, err := h.Handle(context.Background())

	require.NoError(t, err)
	require.Len(t, apps, 1)
	require.Equal(t, "Home", apps[0].DisplayName)
}

func TestListApplications_Handle_BadIcon(t *testing.T) {
	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("List", mock.Anything).Return([]domainrepo.ApplicationRecord{
		{ID: 1, Icon: "invalid-icon", DisplayName: "Bad", Url: "https://example.com"},
	}, nil)

	h := query.NewListApplications(appRepo)
	_, err := h.Handle(context.Background())

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

// ── GetApplication ─────────────────────────────────────────────────────────

func TestGetApplication_Handle_NotFound(t *testing.T) {
	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("Get", mock.Anything, uint(5)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityApplication))

	h := query.NewGetApplication(appRepo)
	_, err := h.Handle(context.Background(), 5)

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}

func TestGetApplication_Handle_Success(t *testing.T) {
	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("Get", mock.Anything, uint(5)).Return(&domainrepo.ApplicationRecord{
		ID:          5,
		Icon:        "mdi:server",
		DisplayName: "Proxmox",
		Url:         "https://proxmox.example.com",
	}, nil)

	h := query.NewGetApplication(appRepo)
	app, err := h.Handle(context.Background(), 5)

	require.NoError(t, err)
	require.Equal(t, "Proxmox", app.DisplayName)
	require.Equal(t, uint(5), app.ID)
}

// ── GetUserApplications ────────────────────────────────────────────────────

func TestGetUserApplications_Handle_FilteredByGroup(t *testing.T) {
	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("List", mock.Anything).Return([]domainrepo.ApplicationRecord{
		{ID: 1, Icon: "mdi:home", DisplayName: "Public", Url: "https://example.com", VisibleToGroups: []string{}},
		{ID: 2, Icon: "mdi:lock", DisplayName: "Admin Only", Url: "https://admin.example.com", VisibleToGroups: []string{"admin"}},
	}, nil)

	listApps := query.NewListApplications(appRepo)
	h := query.NewGetUserApplications(listApps)

	apps, err := h.Handle(context.Background(), []string{"dash_user"})

	require.NoError(t, err)
	require.Len(t, apps, 1)
	require.Equal(t, "Public", apps[0].DisplayName)
}

func TestGetUserApplications_Handle_AdminSeesAll(t *testing.T) {
	appRepo := &repoMock.ApplicationRepository{}
	appRepo.On("List", mock.Anything).Return([]domainrepo.ApplicationRecord{
		{ID: 1, Icon: "mdi:home", DisplayName: "Public", Url: "https://example.com", VisibleToGroups: []string{}},
		{ID: 2, Icon: "mdi:lock", DisplayName: "Admin Only", Url: "https://admin.example.com", VisibleToGroups: []string{"admin"}},
	}, nil)

	listApps := query.NewListApplications(appRepo)
	h := query.NewGetUserApplications(listApps)

	apps, err := h.Handle(context.Background(), []string{"dash_user", "admin"})

	require.NoError(t, err)
	require.Len(t, apps, 2)
}
