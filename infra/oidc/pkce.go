package oidc

import (
	"crypto/rand"
	"encoding/base64"
	"encoding/hex"
	"fmt"
)

// GenerateCodeVerifier generates a random PKCE code_verifier (32 bytes, base64url-encoded).
func GenerateCodeVerifier() (string, error) {
	b := make([]byte, 32)
	if _, err := rand.Read(b); err != nil {
		return "", fmt.Errorf("generating code verifier: %w", err)
	}
	return base64.RawURLEncoding.EncodeToString(b), nil
}

// GenerateState generates a random OAuth2 state parameter (16 bytes, hex-encoded).
func GenerateState() (string, error) {
	b := make([]byte, 16)
	if _, err := rand.Read(b); err != nil {
		return "", fmt.Errorf("generating state: %w", err)
	}
	return hex.EncodeToString(b), nil
}

// StateCookie holds the in-flight OAuth2 state for CSRF protection and PKCE.
// It is stored in a short-lived encrypted cookie during the login flow.
type StateCookie struct {
	State        string `json:"s"`
	CodeVerifier string `json:"cv"`
	ReturnTo     string `json:"rt"`
}
