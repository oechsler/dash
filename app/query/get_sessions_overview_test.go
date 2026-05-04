package query_test

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/query"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

func TestGetSessionsOverview_Handle_RepoError(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("ListByUserID", mock.Anything, "user-1").Return(nil, errors.New("db error"))

	h := query.NewGetSessionsOverview(repo)
	_, err := h.Handle(context.Background(), query.SessionsOverviewInput{UserID: "user-1"})

	require.Error(t, err)
}

func TestGetSessionsOverview_Handle_NoSessions_NoCookie(t *testing.T) {
	repo := &repoMock.SessionRepository{}
	repo.On("ListByUserID", mock.Anything, "user-1").Return([]*domainrepo.SessionRecord{}, nil)

	h := query.NewGetSessionsOverview(repo)
	result, err := h.Handle(context.Background(), query.SessionsOverviewInput{
		UserID:           "user-1",
		CurrentSessionID: "", // no cookie
	})

	require.NoError(t, err)
	require.Empty(t, result.Sessions)
}

func TestGetSessionsOverview_Handle_CurrentSession_InDB(t *testing.T) {
	now := time.Now()
	record := &domainrepo.SessionRecord{
		ID:             "rec-1",
		UserID:         "user-1",
		SessionID:      "session-cookie-abc",
		LastIP:         "1.2.3.4",
		LastAccessedAt: now,
		CreatedAt:      now.Add(-time.Hour),
		ExpiresAt:      now.Add(time.Hour), // still valid
		IssuedAt:       now.Add(-30 * time.Minute),
		UserAgent:      "Mozilla/5.0",
	}

	repo := &repoMock.SessionRepository{}
	repo.On("ListByUserID", mock.Anything, "user-1").Return([]*domainrepo.SessionRecord{record}, nil)

	h := query.NewGetSessionsOverview(repo)
	result, err := h.Handle(context.Background(), query.SessionsOverviewInput{
		UserID:           "user-1",
		CurrentSessionID: "session-cookie-abc",
		CurrentIP:        "1.2.3.4",
	})

	require.NoError(t, err)
	require.Len(t, result.Sessions, 1)
	require.True(t, result.Sessions[0].IsCurrent)
	require.True(t, result.Sessions[0].IsActive)
	require.Equal(t, "rec-1", result.Sessions[0].ID)
}

func TestGetSessionsOverview_Handle_CurrentSession_NotInDB(t *testing.T) {
	// Cookie present but no DB record yet
	repo := &repoMock.SessionRepository{}
	repo.On("ListByUserID", mock.Anything, "user-1").Return([]*domainrepo.SessionRecord{}, nil)

	h := query.NewGetSessionsOverview(repo)
	result, err := h.Handle(context.Background(), query.SessionsOverviewInput{
		UserID:           "user-1",
		CurrentSessionID: "orphan-session",
		CurrentIP:        "1.2.3.4",
		CurrentUserAgent: "Mozilla/5.0",
	})

	require.NoError(t, err)
	require.Len(t, result.Sessions, 1)
	require.True(t, result.Sessions[0].IsCurrent)
	require.True(t, result.Sessions[0].IsActive)
	require.Equal(t, "", result.Sessions[0].ID)
}

func TestGetSessionsOverview_Handle_MultipleOtherSessions(t *testing.T) {
	now := time.Now()
	current := &domainrepo.SessionRecord{
		ID:        "rec-1",
		SessionID: "current-session",
		ExpiresAt: now.Add(time.Hour),
		IssuedAt:  now.Add(-10 * time.Minute),
	}
	other := &domainrepo.SessionRecord{
		ID:        "rec-2",
		SessionID: "other-session",
		ExpiresAt: now.Add(-time.Hour), // expired
		IssuedAt:  now.Add(-2 * time.Hour),
	}

	repo := &repoMock.SessionRepository{}
	repo.On("ListByUserID", mock.Anything, "user-1").
		Return([]*domainrepo.SessionRecord{current, other}, nil)

	h := query.NewGetSessionsOverview(repo)
	result, err := h.Handle(context.Background(), query.SessionsOverviewInput{
		UserID:           "user-1",
		CurrentSessionID: "current-session",
	})

	require.NoError(t, err)
	require.Len(t, result.Sessions, 2)
	require.True(t, result.Sessions[0].IsCurrent)
	require.False(t, result.Sessions[1].IsCurrent)
	require.False(t, result.Sessions[1].IsActive) // expired
}

func TestGetSessionsOverview_Handle_PinnedSession(t *testing.T) {
	now := time.Now()
	pinned := &domainrepo.SessionRecord{
		ID:             "rec-3",
		SessionID:      "pinned-session",
		ExpiresAt:      now.Add(-time.Hour), // token expired
		IssuedAt:       now.Add(-2 * time.Hour),
		LastAccessedAt: now.Add(-10 * time.Minute),
		PinnedUntil:    now.Add(365 * 24 * time.Hour),
	}

	repo := &repoMock.SessionRepository{}
	repo.On("ListByUserID", mock.Anything, "user-1").
		Return([]*domainrepo.SessionRecord{pinned}, nil)

	h := query.NewGetSessionsOverview(repo)
	result, err := h.Handle(context.Background(), query.SessionsOverviewInput{
		UserID:           "user-1",
		CurrentSessionID: "current-session", // different from pinned
	})

	require.NoError(t, err)
	require.Len(t, result.Sessions, 2) // synthetic current + pinned

	// Find the pinned session
	var pinnedItem *query.SessionOverviewItem
	for _, s := range result.Sessions {
		if s.SessionID == "pinned-session" {
			pinnedItem = s
		}
	}
	require.NotNil(t, pinnedItem)
	require.True(t, pinnedItem.IsPinned)
	require.True(t, pinnedItem.IsActive) // pinned and recently accessed
}

func TestGetSessionsOverview_Handle_ZeroIssuedAt_NotActive(t *testing.T) {
	now := time.Now()
	stale := &domainrepo.SessionRecord{
		ID:        "rec-4",
		SessionID: "stale-session",
		ExpiresAt: now.Add(-time.Hour), // expired
		IssuedAt:  time.Time{},         // zero — no active-window check
	}

	repo := &repoMock.SessionRepository{}
	repo.On("ListByUserID", mock.Anything, "user-1").
		Return([]*domainrepo.SessionRecord{stale}, nil)

	h := query.NewGetSessionsOverview(repo)
	result, err := h.Handle(context.Background(), query.SessionsOverviewInput{UserID: "user-1"})

	require.NoError(t, err)
	require.Len(t, result.Sessions, 1)
	require.False(t, result.Sessions[0].IsActive)
}
