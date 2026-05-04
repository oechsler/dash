package repo

import (
	"context"
	"time"
)

// SessionRecord is the data transfer type exchanged with the SessionRepository.
// All identity data (name, email, groups, etc.) is stored server-side so that
// the session cookie only needs to carry the SessionID — the raw ID token is
// never written to the cookie after login.
type SessionRecord struct {
	ID             string
	UserID         string
	SessionID      string    // matches the SessionID stored in the session cookie
	IssuedAt       time.Time // OIDC token iat; used to compute the active-window heuristic
	ExpiresAt      time.Time // OIDC token expiry; used for cleanup and active-window check
	PinnedUntil    time.Time // zero = not pinned; sliding-window expiry when pinned
	LastAccessedAt time.Time // updated on every touch
	LastIP         string
	UserAgent      string
	CreatedAt      time.Time
	// Identity fields — stored at login/refresh time; used by LoadIdentity to
	// reconstruct the domain Identity without touching the OIDC token.
	Sub         string
	Username    string
	Email       string
	FirstName   string
	LastName    string
	DisplayName string
	Picture     string // empty = no picture
	ProfileUrl  string // empty = no profile URL
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
	Unpin(ctx context.Context, recordID string, userID string) error
	// Touch updates LastIP, UserAgent, and LastAccessedAt for the given session and
	// returns the full updated record — nil if the session no longer exists or is
	// not active (expires_at <= now AND pinned_until <= now).
	// When the session is pinned, PinnedUntil is extended by 1 year on every touch.
	Touch(ctx context.Context, sessionID string, lastIP string, userAgent string) (*SessionRecord, error)
	// ListByUserID returns all sessions still relevant for the given user
	// (token valid OR pin still active), newest first.
	ListByUserID(ctx context.Context, userID string) ([]*SessionRecord, error)
	// DeleteByID removes a specific session by its DB record ID, scoped to userID for safety.
	DeleteByID(ctx context.Context, recordID string, userID string) error
	// DeleteBySessionID removes the session with the given cookie SessionID.
	DeleteBySessionID(ctx context.Context, sessionID string) error
	// DeleteByUserID removes all sessions for the given user.
	DeleteByUserID(ctx context.Context, userID string) error
	// RefreshBySessionID updates token timing, groups, and IsAdmin for an existing session.
	RefreshBySessionID(ctx context.Context, record *SessionRecord) error
	// DeleteExpired removes all sessions whose token has expired and that are no
	// longer pinned (or whose pin has also expired).
	DeleteExpired(ctx context.Context) error
}
