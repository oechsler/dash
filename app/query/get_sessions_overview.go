package query

import (
	"context"
	"time"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// SessionsOverviewInput carries the request-context data extracted from the session cookie.
// These are infrastructure values the handler reads and passes in — the app layer is
// not aware of cookies or HTTP.
type SessionsOverviewInput struct {
	UserID           string
	CurrentSessionID string    // from cookie (empty if no cookie)
	CurrentIP        string    // client IP from the request
	CurrentUserAgent string    // User-Agent header from the request
	CurrentExpiresAt time.Time // OIDC token expiry from cookie; zero if session is expired/missing
}

// SessionOverviewItem is the read model for a single session entry.
type SessionOverviewItem struct {
	ID             string // empty for the current session if not yet in DB
	SessionID      string
	LastIP         string
	LastAccessedAt time.Time
	CreatedAt      time.Time
	PinnedUntil    time.Time
	UserAgent      string
	IsActive       bool // true while the OIDC token is still valid (ExpiresAt > now)
	IsCurrent      bool // matches the caller's current session cookie
	IsPinned       bool // PinnedUntil is set; determines Pin vs Unpin button
}

// SessionsOverview is the read model returned by GetSessionsOverview.
// Sessions always has the current session as the first entry (if a cookie is present),
// followed by all other sessions still in scope.
type SessionsOverview struct {
	Sessions []*SessionOverviewItem
}

// UserSessionsOverviewGetter handles the get-sessions-overview query.
type UserSessionsOverviewGetter interface {
	Handle(ctx context.Context, input SessionsOverviewInput) (*SessionsOverview, error)
}

type GetSessionsOverview struct {
	Repo domainrepo.SessionRepository
}

func NewGetSessionsOverview(repo domainrepo.SessionRepository) *GetSessionsOverview {
	return &GetSessionsOverview{Repo: repo}
}

// isSessionActive returns true while the OIDC token is still valid, or — for pinned
// sessions — if the device has been active within one token-window since its last access.
// Token window = exp - iat; a pinned session that goes unused for that duration is stale.
func isSessionActive(r *domainrepo.SessionRecord, now time.Time) bool {
	if r.ExpiresAt.After(now) {
		return true // token still valid
	}
	if r.IssuedAt.IsZero() || r.LastAccessedAt.IsZero() {
		return false
	}
	tokenWindow := r.ExpiresAt.Sub(r.IssuedAt)
	return r.LastAccessedAt.Add(tokenWindow).After(now)
}

func (h *GetSessionsOverview) Handle(ctx context.Context, input SessionsOverviewInput) (*SessionsOverview, error) {
	records, err := h.Repo.ListByUserID(ctx, input.UserID)
	if err != nil {
		return nil, err
	}

	now := time.Now()

	// Find whether the current session is already in the DB.
	var currentRecord *domainrepo.SessionRecord
	for _, r := range records {
		if r.SessionID == input.CurrentSessionID {
			currentRecord = r
			break
		}
	}

	items := make([]*SessionOverviewItem, 0, len(records)+1)

	// Always prepend the current session as the first entry (if a cookie is present).
	if input.CurrentSessionID != "" {
		if currentRecord != nil {
			items = append(items, &SessionOverviewItem{
				ID:             currentRecord.ID,
				SessionID:      currentRecord.SessionID,
				LastIP:         currentRecord.LastIP,
				LastAccessedAt: currentRecord.LastAccessedAt,
				CreatedAt:      currentRecord.CreatedAt,
				PinnedUntil:    currentRecord.PinnedUntil,
				UserAgent:      currentRecord.UserAgent,
				IsActive:       isSessionActive(currentRecord, now),
				IsCurrent:      true,
				IsPinned:       !currentRecord.PinnedUntil.IsZero(),
			})
		} else {
			// Current session not yet in DB (e.g. repo not wired up) — show it with a Pin button.
			items = append(items, &SessionOverviewItem{
				SessionID:      input.CurrentSessionID,
				LastIP:         input.CurrentIP,
				LastAccessedAt: now,
				UserAgent:      input.CurrentUserAgent,
				IsActive:       true,
				IsCurrent:      true,
				IsPinned:       false,
			})
		}
	}

	// Append all other sessions (skip the current one — already added above).
	for _, r := range records {
		if r.SessionID == input.CurrentSessionID {
			continue
		}
		items = append(items, &SessionOverviewItem{
			ID:             r.ID,
			SessionID:      r.SessionID,
			LastIP:         r.LastIP,
			LastAccessedAt: r.LastAccessedAt,
			CreatedAt:      r.CreatedAt,
			PinnedUntil:    r.PinnedUntil,
			UserAgent:      r.UserAgent,
			IsActive:       isSessionActive(r, now),
			IsCurrent:      false,
			IsPinned:       !r.PinnedUntil.IsZero(),
		})
	}

	return &SessionsOverview{Sessions: items}, nil
}
