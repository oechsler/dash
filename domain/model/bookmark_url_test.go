package model

import "testing"

func TestParseBookmarkURL(t *testing.T) {
	tests := []struct {
		input    string
		wantErr  bool
		wantHost string
	}{
		{"https://example.com", false, "example.com"},
		{"http://example.com/path", false, "example.com"},
		{"https://example.com:8080/path?q=1", false, "example.com:8080"},
		{"https://sub.example.com", false, "sub.example.com"},
		{"http://localhost:3000", false, "localhost:3000"},
		{"", true, ""},
		{"/relative/path", true, ""},
		{"not-a-url", true, ""},
		{"ftp://example.com", false, "example.com"}, // valid URL with host
		{"//example.com", true, ""},                 // no scheme — url.ParseRequestURI fails
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			u, err := ParseBookmarkURL(tt.input)
			if tt.wantErr {
				if err == nil {
					t.Errorf("ParseBookmarkURL(%q) expected error, got nil", tt.input)
				}
				if !u.IsZero() {
					t.Errorf("ParseBookmarkURL(%q) on error: expected zero BookmarkURL", tt.input)
				}
			} else {
				if err != nil {
					t.Errorf("ParseBookmarkURL(%q) unexpected error: %v", tt.input, err)
				}
				if u.String() != tt.input {
					t.Errorf("String() = %q, want %q", u.String(), tt.input)
				}
				if u.Host() != tt.wantHost {
					t.Errorf("Host() = %q, want %q", u.Host(), tt.wantHost)
				}
				if u.IsZero() {
					t.Errorf("ParseBookmarkURL(%q) should not be zero", tt.input)
				}
			}
		})
	}
}

func TestBookmarkURLZeroValue(t *testing.T) {
	var u BookmarkURL
	if !u.IsZero() {
		t.Error("zero BookmarkURL should be zero")
	}
	if u.String() != "" {
		t.Errorf("zero BookmarkURL.String() = %q, want \"\"", u.String())
	}
	if u.Host() != "" {
		t.Errorf("zero BookmarkURL.Host() = %q, want \"\"", u.Host())
	}
}
