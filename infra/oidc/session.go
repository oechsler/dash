package oidc

import (
	"github.com/oechsler-it/dash/domain/model"
	"github.com/oechsler-it/dash/config"
	"encoding/hex"
	"fmt"
	"time"

	"github.com/gofiber/fiber/v2"
	"github.com/gorilla/securecookie"
)

const stateCookieName = "dash-oidc-state"

// SessionData is the serialisation container stored in the encrypted session cookie.
// It maps 1:1 to model.Identity plus the infrastructure fields needed for logout.
type SessionData struct {
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
	codec      *securecookie.SecureCookie
	cookieName string
	domain     string
	secure     bool
	maxAge     int
}

// NewSessionStore creates a SessionStore from cookie configuration.
// HashKey (64 bytes hex) and BlockKey (32 bytes hex) are required.
func NewSessionStore(cfg *config.OIDCCookieConfig) (*SessionStore, error) {
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
		codec:      securecookie.New(hashKey, blockKey),
		cookieName: cfg.Name,
		domain:     cfg.Domain,
		secure:     cfg.Secure,
		maxAge:     cfg.MaxAge,
	}, nil
}

// Save encodes the identity into an encrypted session cookie.
func (s *SessionStore) Save(c *fiber.Ctx, identity model.Identity, rawIDToken string, expiresAt int64) error {
	data := SessionData{
		Sub:        identity.UserID,
		FirstName:  identity.FirstName,
		LastName:   identity.LastName,
		DisplayName: identity.DisplayName,
		Username:   identity.Username,
		Email:      identity.Email,
		Groups:     identity.Groups,
		IsAdmin:    identity.IsAdmin,
		RawIDToken: rawIDToken,
		ExpiresAt:  expiresAt,
	}
	if identity.Picture != nil {
		data.Picture = *identity.Picture
	}
	if identity.ProfileUrl != nil {
		data.ProfileUrl = *identity.ProfileUrl
	}

	encoded, err := s.codec.Encode(s.cookieName, &data)
	if err != nil {
		return fmt.Errorf("encoding session: %w", err)
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
	return nil
}

// Load decodes the session cookie. Returns false if missing, invalid, or expired.
func (s *SessionStore) Load(c *fiber.Ctx) (SessionData, bool) {
	encoded := c.Cookies(s.cookieName)
	if encoded == "" {
		return SessionData{}, false
	}

	var data SessionData
	if err := s.codec.Decode(s.cookieName, encoded, &data); err != nil {
		return SessionData{}, false
	}

	if data.ExpiresAt > 0 && time.Now().Unix() > data.ExpiresAt {
		return SessionData{}, false
	}

	return data, true
}

// LoadIdentity decodes the session cookie and returns the domain Identity.
// This method implements the middleware.IdentityLoader interface.
func (s *SessionStore) LoadIdentity(c *fiber.Ctx) (model.Identity, bool) {
	data, ok := s.Load(c)
	if !ok {
		return model.Identity{}, false
	}
	return data.ToIdentity(), true
}

// Clear instructs the browser to delete the session cookie.
func (s *SessionStore) Clear(c *fiber.Ctx) {
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
func (s *SessionStore) SaveStateCookie(c *fiber.Ctx, sc StateCookie) error {
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
func (s *SessionStore) LoadAndClearStateCookie(c *fiber.Ctx) (StateCookie, error) {
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
