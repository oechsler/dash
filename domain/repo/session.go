package repo

import (
	"context"
	"time"
)

// SessionRecord is the data transfer type exchanged with the SessionRepository.
// It stores all sessions — both regular (token-backed) and pinned (fallback-capable).
// Identity fields are cached here so pinned sessions can authenticate after the OIDC
// token expires, without an IDP round-trip.
type SessionRecord struct {
	ID             string
	UserID         string
	SessionID      string    // matches the SessionID stored in the session cookie
	IssuedAt       time.Time // OIDC token iat (nbf proxy; used to compute token window)
	ExpiresAt      time.Time // OIDC token expiry
	PinnedUntil    time.Time // zero = not pinned; sliding-window expiry when pinned
	LastAccessedAt time.Time // updated on every pinned-fallback load
	LastIP         string
	UserAgent      string
	CreatedAt      time.Time
	// Identity fields cached for pinned-session fallback auth after token expiry.
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

// SessionRepository manages all user sessions.
type SessionRepository interface {
	// Create stores a new session record. Silently ignores duplicate SessionIDs.
	Create(ctx context.Context, record *SessionRecord) error
	// Pin sets PinnedUntil on a session, enabling token-expired fallback auth.
	Pin(ctx context.Context, sessionID string, userID string, pinnedUntil time.Time) error
	// Unpin clears PinnedUntil (sets it to zero), disabling fallback auth.
	// The record is kept alive as long as the OIDC token is valid.
	Unpin(ctx context.Context, id string, userID string) error
	// GetBySessionID returns an actively pinned session (PinnedUntil > now).
	// Used exclusively for fallback auth after token expiry.
	// Returns NotFound if the session does not exist or is not currently pinned.
	GetBySessionID(ctx context.Context, sessionID string) (*SessionRecord, error)
	// ExistsBySessionID returns true if any record with the given SessionID exists,
	// regardless of pin status. Used to detect invalidated (deleted) sessions.
	ExistsBySessionID(ctx context.Context, sessionID string) (bool, error)
	// Touch updates LastIP, UserAgent, and LastAccessedAt for the given session.
	// Returns false if the session no longer exists (was invalidated).
	// Used on every authenticated request to keep access metadata current.
	Touch(ctx context.Context, sessionID string, lastIP string, userAgent string) (bool, error)
	// TouchBySessionID extends PinnedUntil and records the latest access metadata.
	TouchBySessionID(ctx context.Context, sessionID string, newPinnedUntil time.Time, lastIP string, userAgent string) error
	// ListByUserID returns all sessions still relevant for the given user
	// (token valid OR pin still active), newest first.
	ListByUserID(ctx context.Context, userID string) ([]*SessionRecord, error)
	// DeleteByID removes a specific session, scoped to userID for safety.
	// Used for forced sign-out / session invalidation from another device.
	DeleteByID(ctx context.Context, id string, userID string) error
	// DeleteBySessionID removes the session with the given cookie SessionID.
	// Used on voluntary logout where the cookie is still readable.
	DeleteBySessionID(ctx context.Context, sessionID string) error
	// DeleteByUserID removes all sessions for the given user.
	DeleteByUserID(ctx context.Context, userID string) error
	// DeleteExpired removes all sessions whose token has expired and that are no
	// longer pinned (or whose pin has also expired).
	DeleteExpired(ctx context.Context) error
}
