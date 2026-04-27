// Package transfer contains shared data-transfer types used by both
// app/query and app/command to avoid import cycles.
package transfer

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"strings"
	"time"
)

// UserDataExport is the top-level structure for exported user data.
// Signature (when present) is the HMAC-SHA256 over the canonical JSON of this
// struct with the Signature field set to "" (omitempty → absent).
type UserDataExport struct {
	Version      int                 `json:"version"`
	ExportedAt   time.Time           `json:"exported_at"`
	Username     string              `json:"username"`
	Settings     SettingsExport      `json:"settings"`
	Themes       []ThemeExport       `json:"themes"`
	Categories   []CategoryExport    `json:"categories"`
	Applications []ApplicationExport `json:"applications,omitempty"`
	Signature    string              `json:"signature,omitempty"`
}

type SettingsExport struct {
	Language  string `json:"language"`
	Timezone  string `json:"timezone"`
	ThemeName string `json:"theme_name,omitempty"`
}

type ThemeExport struct {
	Hash      string `json:"hash"`
	Name      string `json:"name"`
	Primary   string `json:"primary"`
	Secondary string `json:"secondary"`
	Tertiary  string `json:"tertiary"`
}

type CategoryExport struct {
	Hash        string           `json:"hash"`
	DisplayName string           `json:"display_name"`
	IsShelved   bool             `json:"is_shelved"`
	Bookmarks   []BookmarkExport `json:"bookmarks"`
}

type BookmarkExport struct {
	Hash        string `json:"hash"`
	Icon        string `json:"icon"`
	DisplayName string `json:"display_name"`
	URL         string `json:"url"`
}

type ApplicationExport struct {
	Hash            string   `json:"hash"`
	Icon            string   `json:"icon"`
	DisplayName     string   `json:"display_name"`
	URL             string   `json:"url"`
	VisibleToGroups []string `json:"visible_to_groups"`
}

// ContentHash computes a stable 16-char hex hash from the given parts.
func ContentHash(parts ...string) string {
	h := sha256.Sum256([]byte(strings.Join(parts, "\x00")))
	return hex.EncodeToString(h[:])[:16]
}

// ErrInvalidSignature is returned by UnmarshalExport when the checksum does not match.
var ErrInvalidSignature = errors.New("export signature is invalid")

// ErrMissingSignature is returned when the file has no signature field.
var ErrMissingSignature = errors.New("export has no signature")

// canonical returns the compact JSON of export with Signature cleared.
// This is the byte sequence that is hashed.
func canonical(export *UserDataExport) ([]byte, error) {
	sig := export.Signature
	export.Signature = ""
	b, err := json.Marshal(export)
	export.Signature = sig
	return b, err
}

// MarshalExport serialises export to pretty-printed JSON with an embedded
// SHA-256 checksum in the "signature" field. No secret is required — the
// checksum is derived solely from the content and can be verified by any instance.
func MarshalExport(export *UserDataExport) ([]byte, error) {
	export.Signature = ""
	b, err := canonical(export)
	if err != nil {
		return nil, err
	}
	h := sha256.Sum256(b)
	export.Signature = hex.EncodeToString(h[:])
	return json.MarshalIndent(export, "", "  ")
}

// UnmarshalExport parses a file produced by MarshalExport.
// The signature field is mandatory; files without one are rejected with ErrMissingSignature.
func UnmarshalExport(data []byte) (*UserDataExport, error) {
	var export UserDataExport
	if err := json.Unmarshal(data, &export); err != nil {
		return nil, err
	}

	if export.Signature == "" {
		return nil, ErrMissingSignature
	}

	b, err := canonical(&export)
	if err != nil {
		return nil, err
	}
	h := sha256.Sum256(b)
	if hex.EncodeToString(h[:]) != export.Signature {
		return nil, ErrInvalidSignature
	}

	export.Signature = ""
	return &export, nil
}
