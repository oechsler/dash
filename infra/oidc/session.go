package oidc

import (
	"context"
	"encoding/hex"
	"fmt"
	"time"

	"git.at.oechsler.it/samuel/dash/v2/config"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	"git.at.oechsler.it/samuel/dash/v2/domain/model"

	"github.com/gofiber/fiber/v3"
	"github.com/google/uuid"
	"github.com/gorilla/securecookie"
)

const stateCookieName = "dash-oidc-state"

// SessionData is the minimal payload stored in the encrypted session cookie.
//   - SessionID      — used for server-side session revocation and DB lookup.
//   - RawIDToken     — kept solely for id_token_hint at OIDC logout.
//     Omitted (empty) when it would push the cookie over cookieSizeBudget.
//     Never used for identity or expiry checks — the DB is the authority.
//   - CookieIssuedAt — unix timestamp of the last time the cookie was written;
//     used to throttle re-issuance so the cookie is only refreshed when it is
//     within cookieRenewalThreshold of its pinnedSessionMaxAge expiry.
type SessionData struct {
	SessionID      string `json:"sid"`
	RawIDToken     string `json:"idt,omitempty"`
	CookieIssuedAt int64  `json:"cia"`
}

// SessionStore stores and retrieves the session from an encrypted cookie.
// It also manages the short-lived state cookie used during the login flow.
type SessionStore struct {
	codec       *securecookie.SecureCookie
	cookieName  string
	domain      string
	secure      bool
	maxAge      int
	sessionRepo domainrepo.SessionRepository // optional; enables session fallback and revocation
}

// NewSessionStore creates a SessionStore from cookie configuration.
// HashKey (64 bytes hex) and BlockKey (32 bytes hex) are required.
// sessionRepo is optional; pass nil to disable DB-backed session features.
func NewSessionStore(cfg *config.OIDCCookieConfig, sessionRepo domainrepo.SessionRepository) (*SessionStore, error) {
	hashKey, err := hex.DecodeString(cfg.HashKey)
	if err != nil {
		return nil, fmt.Errorf("OIDC_COOKIE_HASH_KEY is not valid hex: %w", err)
	}
	if len(hashKey) != 64 {
		return nil, fmt.Errorf("OIDC_COOKIE_HASH_KEY must be 64 bytes (128 hex chars), got %d bytes", len(hashKey))
	}

	blockKey, err := hex.DecodeString(cfg.BlockKey)
	if err != nil {
		return nil, fmt.Errorf("OIDC_COOKIE_BLOCK_KEY is not valid hex: %w", err)
	}
	if len(blockKey) != 32 {
		return nil, fmt.Errorf("OIDC_COOKIE_BLOCK_KEY must be 32 bytes (64 hex chars), got %d bytes", len(blockKey))
	}

	return &SessionStore{
		codec:       securecookie.New(hashKey, blockKey),
		cookieName:  cfg.Name,
		domain:      cfg.Domain,
		secure:      cfg.Secure,
		maxAge:      cfg.MaxAge,
		sessionRepo: sessionRepo,
	}, nil
}

// Save writes a new session cookie containing a fresh SessionID and the raw ID token.
// The caller is responsible for persisting the session (including identity) to the DB.
func (s *SessionStore) Save(c fiber.Ctx, rawIDToken string) (SessionData, error) {
	data := SessionData{
		SessionID:  uuid.New().String(),
		RawIDToken: rawIDToken,
	}
	if err := s.writeCookie(c, data, s.maxAge); err != nil {
		return SessionData{}, err
	}
	return data, nil
}

// SaveWithID re-issues the session cookie with an existing SessionID.
// Used during the group-refresh flow so the session identifier stays stable.
// If persist is true the cookie receives a 1-year MaxAge (for pinned sessions).
func (s *SessionStore) SaveWithID(c fiber.Ctx, rawIDToken string, sessionID string, persist bool) (SessionData, error) {
	data := SessionData{
		SessionID:  sessionID,
		RawIDToken: rawIDToken,
	}
	maxAge := s.maxAge
	if persist {
		maxAge = pinnedSessionMaxAge
	}
	if err := s.writeCookie(c, data, maxAge); err != nil {
		return SessionData{}, err
	}
	return data, nil
}

// Load decodes the session cookie. Returns false only if the cookie is missing
// or cryptographically invalid. Session validity is determined by the DB, not
// the token's exp claim.
func (s *SessionStore) Load(c fiber.Ctx) (SessionData, bool) {
	return s.loadRaw(c)
}

// LoadExpired decodes the session cookie without any expiry checks.
// Identical to Load; kept for call-site clarity in flows where token expiry
// is expected (e.g. refresh, unpin).
func (s *SessionStore) LoadExpired(c fiber.Ctx) (SessionData, bool) {
	return s.loadRaw(c)
}

func (s *SessionStore) loadRaw(c fiber.Ctx) (SessionData, bool) {
	encoded := c.Cookies(s.cookieName)
	if encoded == "" {
		return SessionData{}, false
	}
	var data SessionData
	if err := s.codec.Decode(s.cookieName, encoded, &data); err != nil {
		return SessionData{}, false
	}
	return data, true
}

// LoadIdentity resolves the current user's identity entirely from the DB.
// It calls Touch to verify the session is still active, slide PinnedUntil,
// and retrieve the stored identity fields — the cookie carries only the SessionID.
// This method implements the middleware.IdentityLoader interface.
func (s *SessionStore) LoadIdentity(c fiber.Ctx) (model.Identity, bool) {
	data, ok := s.loadRaw(c)
	if !ok || data.SessionID == "" {
		return model.Identity{}, false
	}
	if s.sessionRepo == nil {
		return model.Identity{}, false
	}

	record, err := s.sessionRepo.Touch(context.Background(), data.SessionID, c.IP(), c.Get("User-Agent"))
	if err != nil {
		// DB error: fail closed — with server-side identity we cannot reconstruct
		// the user without the DB, so we must deny rather than guess.
		return model.Identity{}, false
	}
	if record == nil {
		return model.Identity{}, false // session not found or fully expired
	}

	if record.PinnedUntil.After(time.Now()) {
		c.Locals("session_pinned", true)
		_ = s.PersistCookie(c)
	}

	return recordToIdentity(record), true
}

// recordToIdentity maps a SessionRecord's stored identity fields to a domain Identity.
func recordToIdentity(r *domainrepo.SessionRecord) model.Identity {
	var picture *string
	if r.Picture != "" {
		picture = &r.Picture
	}
	var profileUrl *string
	if r.ProfileUrl != "" {
		profileUrl = &r.ProfileUrl
	}
	return model.Identity{
		UserID:      r.UserID,
		FirstName:   r.FirstName,
		LastName:    r.LastName,
		DisplayName: r.DisplayName,
		Username:    r.Username,
		Email:       r.Email,
		Picture:     picture,
		Groups:      r.Groups,
		IsAdmin:     r.IsAdmin,
		ProfileUrl:  profileUrl,
	}
}

// pinnedSessionMaxAge is the cookie lifetime used when a session is pinned.
const pinnedSessionMaxAge = 365 * 24 * 60 * 60 // 1 year in seconds

// cookieRenewalThreshold is how far before cookie expiry we re-issue it.
// The cookie is only rewritten when less than this much time remains, avoiding
// a Set-Cookie header on every response for active pinned sessions.
const cookieRenewalThreshold = 183 * 24 * time.Hour // ~6 months

// needsRenewal reports whether a pinned session cookie should be re-issued.
// Returns true when CookieIssuedAt is absent (legacy cookie) or when the cookie
// expires within cookieRenewalThreshold.
func needsRenewal(data SessionData) bool {
	if data.CookieIssuedAt == 0 {
		return true
	}
	expiresAt := time.Unix(data.CookieIssuedAt, 0).Add(pinnedSessionMaxAge * time.Second)
	return time.Until(expiresAt) < cookieRenewalThreshold
}

// PersistCookie re-issues the session cookie with pinnedSessionMaxAge when the
// cookie is within cookieRenewalThreshold of expiry (or has no issue timestamp).
// This avoids sending Set-Cookie on every response for active pinned sessions.
func (s *SessionStore) PersistCookie(c fiber.Ctx) error {
	data, ok := s.loadRaw(c)
	if !ok {
		return fmt.Errorf("no session cookie to persist")
	}
	if !needsRenewal(data) {
		return nil
	}
	return s.writeCookie(c, data, pinnedSessionMaxAge)
}

// RevertCookie re-issues the session cookie with the originally configured MaxAge,
// undoing the 1-year lifetime set by PersistCookie.
func (s *SessionStore) RevertCookie(c fiber.Ctx) {
	data, ok := s.loadRaw(c)
	if !ok {
		return
	}
	_ = s.writeCookie(c, data, s.maxAge)
}

// Clear instructs the browser to delete the session cookie.
func (s *SessionStore) Clear(c fiber.Ctx) {
	c.Cookie(&fiber.Cookie{
		Name:     s.cookieName,
		Value:    "",
		Domain:   s.domain,
		MaxAge:   -1,
		Secure:   s.secure,
		HTTPOnly: true,
		SameSite: "Lax",
	})
}

func (s *SessionStore) writeCookie(c fiber.Ctx, data SessionData, maxAge int) error {
	data.CookieIssuedAt = time.Now().Unix()
	encoded, err := s.codec.Encode(s.cookieName, &data)
	if err != nil && data.RawIDToken != "" {
		// securecookie returns ErrValueTooLong when the encoded value exceeds its
		// MaxLength (4096 by default) — this happens for users with many group
		// memberships. Drop the token and retry: identity is always loaded from the
		// DB anyway; the token is only kept for id_token_hint at OIDC logout.
		data.RawIDToken = ""
		encoded, err = s.codec.Encode(s.cookieName, &data)
	}
	if err != nil {
		return fmt.Errorf("encoding session: %w", err)
	}
	c.Cookie(&fiber.Cookie{
		Name:     s.cookieName,
		Value:    encoded,
		Domain:   s.domain,
		MaxAge:   maxAge,
		Secure:   s.secure,
		HTTPOnly: true,
		SameSite: "Lax",
	})
	return nil
}

// SaveStateCookie stores the PKCE state in a short-lived encrypted cookie.
func (s *SessionStore) SaveStateCookie(c fiber.Ctx, sc StateCookie) error {
	encoded, err := s.codec.Encode(stateCookieName, &sc)
	if err != nil {
		return fmt.Errorf("encoding state cookie: %w", err)
	}
	c.Cookie(&fiber.Cookie{
		Name:     stateCookieName,
		Value:    encoded,
		MaxAge:   300, // 5 minutes
		Path:     "/",
		Secure:   s.secure,
		HTTPOnly: true,
		SameSite: "Lax",
	})
	return nil
}

// LoadAndClearStateCookie reads the state cookie and immediately deletes it to prevent replay.
func (s *SessionStore) LoadAndClearStateCookie(c fiber.Ctx) (StateCookie, error) {
	encoded := c.Cookies(stateCookieName)
	if encoded == "" {
		return StateCookie{}, fmt.Errorf("state cookie missing")
	}

	c.Cookie(&fiber.Cookie{
		Name:     stateCookieName,
		Value:    "",
		MaxAge:   -1,
		Path:     "/",
		Secure:   s.secure,
		HTTPOnly: true,
		SameSite: "Lax",
	})

	var sc StateCookie
	if err := s.codec.Decode(stateCookieName, encoded, &sc); err != nil {
		return StateCookie{}, fmt.Errorf("decoding state cookie: %w", err)
	}
	return sc, nil
}
