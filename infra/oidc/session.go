package oidc

import (
	"context"
	"encoding/hex"
	"errors"
	"fmt"
	"time"

	"git.at.oechsler.it/samuel/dash/v2/config"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	"git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"

	"github.com/gofiber/fiber/v3"
	"github.com/google/uuid"
	"github.com/gorilla/securecookie"
)

const stateCookieName = "dash-oidc-state"

// SessionData is the serialisation container stored in the encrypted session cookie.
// It maps 1:1 to model.Identity plus the infrastructure fields needed for logout.
type SessionData struct {
	SessionID   string   `json:"sid"`             // random UUID, stable for the lifetime of this cookie
	Sub         string   `json:"sub"`
	FirstName   string   `json:"fn"`
	LastName    string   `json:"ln"`
	DisplayName string   `json:"dn"`
	Username    string   `json:"un"`
	Email       string   `json:"em"`
	Picture     string   `json:"pic"`
	Groups      []string `json:"grp"`
	IsAdmin     bool     `json:"adm"`
	ProfileUrl  string   `json:"pu"`
	RawIDToken  string   `json:"idt"` // stored for id_token_hint on logout
	ExpiresAt   int64    `json:"exp"` // unix seconds; from ID token exp claim
}

// ToIdentity converts the infrastructure SessionData into the domain Identity value object.
func (d SessionData) ToIdentity() model.Identity {
	id := model.Identity{
		UserID:      d.Sub,
		FirstName:   d.FirstName,
		LastName:    d.LastName,
		DisplayName: d.DisplayName,
		Username:    d.Username,
		Email:       d.Email,
		Groups:      d.Groups,
		IsAdmin:     d.IsAdmin,
	}
	if d.Picture != "" {
		p := d.Picture
		id.Picture = &p
	}
	if d.ProfileUrl != "" {
		pu := d.ProfileUrl
		id.ProfileUrl = &pu
	}
	return id
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

// Save encodes the identity into an encrypted session cookie.
// A fresh SessionID (UUID) is generated for every new session.
// The caller is responsible for persisting the session to the DB via CreateSession.
func (s *SessionStore) Save(c fiber.Ctx, identity model.Identity, rawIDToken string, expiresAt int64) (SessionData, error) {
	data := SessionData{
		SessionID:   uuid.New().String(),
		Sub:         identity.UserID,
		FirstName:   identity.FirstName,
		LastName:    identity.LastName,
		DisplayName: identity.DisplayName,
		Username:    identity.Username,
		Email:       identity.Email,
		Groups:      identity.Groups,
		IsAdmin:     identity.IsAdmin,
		RawIDToken:  rawIDToken,
		ExpiresAt:   expiresAt,
	}
	if identity.Picture != nil {
		data.Picture = *identity.Picture
	}
	if identity.ProfileUrl != nil {
		data.ProfileUrl = *identity.ProfileUrl
	}

	encoded, err := s.codec.Encode(s.cookieName, &data)
	if err != nil {
		return SessionData{}, fmt.Errorf("encoding session: %w", err)
	}

	c.Cookie(&fiber.Cookie{
		Name:     s.cookieName,
		Value:    encoded,
		Domain:   s.domain,
		MaxAge:   s.maxAge,
		Secure:   s.secure,
		HTTPOnly: true,
		SameSite: "Lax",
	})

	return data, nil
}

// Load decodes the session cookie. Returns false if missing, invalid, or expired.
func (s *SessionStore) Load(c fiber.Ctx) (SessionData, bool) {
	data, ok := s.loadRaw(c)
	if !ok {
		return SessionData{}, false
	}
	if data.ExpiresAt > 0 && time.Now().Unix() > data.ExpiresAt {
		return SessionData{}, false
	}
	return data, true
}

// LoadExpired decodes the session cookie ignoring token expiry.
// Returns false only if the cookie is missing or cryptographically invalid.
// Use this to retrieve the SessionID for pinned-session lookup after expiry.
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

// LoadIdentity decodes the session cookie and returns the domain Identity.
// If the session cookie is expired but a matching pinned session exists in the DB,
// the identity is restored from there — no IDP redirect needed.
// This method implements the middleware.IdentityLoader interface.
func (s *SessionStore) LoadIdentity(c fiber.Ctx) (model.Identity, bool) {
	data, ok := s.Load(c)
	if ok {
		// If a session repo is wired up, verify the session record still exists.
		// Deleting the record is how we invalidate a session from another device.
		// Fail open on DB errors so a transient outage doesn't lock everyone out.
		if s.sessionRepo != nil && data.SessionID != "" {
			exists, pinnedUntil, err := s.sessionRepo.Touch(context.Background(), data.SessionID, c.IP(), c.Get("User-Agent"))
			if err == nil && !exists {
				return model.Identity{}, false // record deleted → session invalidated
			}
			// If the session is pinned, re-issue the cookie so the browser's max-age
			// countdown is reset on every request (sliding window on the client side too).
			if err == nil && pinnedUntil.After(time.Now()) {
				_ = s.PersistCookie(c)
			}
			// DB error: fail open
		}
		return data.ToIdentity(), true
	}

	// Pinned-session fallback: load cookie even if expired to get the SessionID.
	if s.sessionRepo != nil {
		expired, ok := s.loadRaw(c)
		if ok && expired.SessionID != "" {
			pinned, err := s.sessionRepo.GetBySessionID(context.Background(), expired.SessionID)
			if err == nil {
				// Sliding-window renewal: extend PinnedUntil by 1 year on every
				// fallback load. If unused for 1 year the record expires naturally.
				newUntil := time.Now().Add(pinnedSessionMaxAge * time.Second)
				_ = s.sessionRepo.TouchBySessionID(context.Background(), expired.SessionID, newUntil, c.IP(), c.Get("User-Agent"))
				// Re-issue cookie with 1-year MaxAge so the browser keeps it.
				_ = s.PersistCookie(c)
				return sessionRecordToIdentity(pinned), true
			}
			// Not found or DB error → fall through to unauthenticated
			var nfe *domainerrors.NotFoundError
			if !errors.As(err, &nfe) {
				// Log DB errors but don't fail the request — treat as unauthenticated
				_ = err
			}
		}
	}

	return model.Identity{}, false
}

// sessionRecordToIdentity converts a SessionRecord into a domain Identity.
func sessionRecordToIdentity(r *domainrepo.SessionRecord) model.Identity {
	id := model.Identity{
		UserID:      r.Sub,
		FirstName:   r.FirstName,
		LastName:    r.LastName,
		DisplayName: r.DisplayName,
		Username:    r.Username,
		Email:       r.Email,
		Groups:      r.Groups,
		IsAdmin:     r.IsAdmin,
	}
	if r.Picture != "" {
		p := r.Picture
		id.Picture = &p
	}
	if r.ProfileUrl != "" {
		pu := r.ProfileUrl
		id.ProfileUrl = &pu
	}
	return id
}

// pinnedSessionMaxAge is the cookie lifetime used when a session is pinned.
// The cookie must outlive the OIDC token so the SessionID is still readable
// after expiry, allowing the pinned-session DB lookup to skip the IDP redirect.
const pinnedSessionMaxAge = 365 * 24 * 60 * 60 // 1 year in seconds

// PersistCookie re-issues the existing session cookie with pinnedSessionMaxAge,
// so the browser keeps it across restarts even when MaxAge is normally 0.
// If the OIDC token is already expired the RawIDToken is stripped — it can no
// longer be used for RP-initiated logout and there is no reason to keep it.
// Call this immediately after pinning a session.
func (s *SessionStore) PersistCookie(c fiber.Ctx) error {
	data, ok := s.loadRaw(c)
	if !ok {
		return fmt.Errorf("no session cookie to persist")
	}
	if data.ExpiresAt > 0 && time.Now().Unix() > data.ExpiresAt {
		data.RawIDToken = ""
	}
	encoded, err := s.codec.Encode(s.cookieName, &data)
	if err != nil {
		return fmt.Errorf("persisting session cookie: %w", err)
	}
	c.Cookie(&fiber.Cookie{
		Name:     s.cookieName,
		Value:    encoded,
		Domain:   s.domain,
		MaxAge:   pinnedSessionMaxAge,
		Secure:   s.secure,
		HTTPOnly: true,
		SameSite: "Lax",
	})
	return nil
}

// RevertCookie re-issues the session cookie with the originally configured MaxAge,
// undoing the 1-year lifetime set by PersistCookie when a session was pinned.
// Call this immediately after unpinning a session.
func (s *SessionStore) RevertCookie(c fiber.Ctx) {
	data, ok := s.loadRaw(c)
	if !ok {
		return
	}
	encoded, err := s.codec.Encode(s.cookieName, &data)
	if err != nil {
		return
	}
	c.Cookie(&fiber.Cookie{
		Name:     s.cookieName,
		Value:    encoded,
		Domain:   s.domain,
		MaxAge:   s.maxAge,
		Secure:   s.secure,
		HTTPOnly: true,
		SameSite: "Lax",
	})
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

	// Delete immediately to prevent replay
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
