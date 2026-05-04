package oidc

import (
	"encoding/base64"
	"encoding/hex"
	"testing"
)

func TestGenerateCodeVerifier(t *testing.T) {
	v1, err := GenerateCodeVerifier()
	if err != nil {
		t.Fatalf("GenerateCodeVerifier() error: %v", err)
	}
	if v1 == "" {
		t.Error("GenerateCodeVerifier() must return a non-empty string")
	}

	// Must be valid base64url (no padding)
	decoded, err := base64.RawURLEncoding.DecodeString(v1)
	if err != nil {
		t.Errorf("GenerateCodeVerifier() result %q is not valid base64url: %v", v1, err)
	}
	if len(decoded) != 32 {
		t.Errorf("GenerateCodeVerifier() decoded length = %d, want 32", len(decoded))
	}

	// Each call must produce a unique value
	v2, err := GenerateCodeVerifier()
	if err != nil {
		t.Fatalf("GenerateCodeVerifier() second call error: %v", err)
	}
	if v1 == v2 {
		t.Error("GenerateCodeVerifier() must return unique values on each call")
	}
}

func TestGenerateState(t *testing.T) {
	s1, err := GenerateState()
	if err != nil {
		t.Fatalf("GenerateState() error: %v", err)
	}
	if s1 == "" {
		t.Error("GenerateState() must return a non-empty string")
	}

	// Must be valid hex-encoded
	decoded, err := hex.DecodeString(s1)
	if err != nil {
		t.Errorf("GenerateState() result %q is not valid hex: %v", s1, err)
	}
	if len(decoded) != 16 {
		t.Errorf("GenerateState() decoded length = %d, want 16", len(decoded))
	}

	// Each call must produce a unique value
	s2, err := GenerateState()
	if err != nil {
		t.Fatalf("GenerateState() second call error: %v", err)
	}
	if s1 == s2 {
		t.Error("GenerateState() must return unique values on each call")
	}
}

func TestStateCookie_Fields(t *testing.T) {
	sc := StateCookie{
		State:            "abc",
		CodeVerifier:     "verifier",
		ReturnTo:         "/dashboard",
		RefreshSessionID: "session-123",
	}
	if sc.State != "abc" {
		t.Errorf("State = %q, want \"abc\"", sc.State)
	}
	if sc.CodeVerifier != "verifier" {
		t.Errorf("CodeVerifier = %q, want \"verifier\"", sc.CodeVerifier)
	}
	if sc.ReturnTo != "/dashboard" {
		t.Errorf("ReturnTo = %q, want \"/dashboard\"", sc.ReturnTo)
	}
	if sc.RefreshSessionID != "session-123" {
		t.Errorf("RefreshSessionID = %q, want \"session-123\"", sc.RefreshSessionID)
	}
}

func TestStateCookie_ZeroValue(t *testing.T) {
	var sc StateCookie
	if sc.RefreshSessionID != "" {
		t.Error("RefreshSessionID zero value should be empty")
	}
}
