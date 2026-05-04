package model

import "testing"

func TestParseColor(t *testing.T) {
	tests := []struct {
		input   string
		wantErr bool
	}{
		{"#fff", false},
		{"#FFF", false},
		{"#aabbcc", false},
		{"#AABBCC", false},
		{"#1e1e2e", false},
		{"#cba6f7", false},
		{"", true},
		{"fff", true},
		{"#gg0000", true},
		{"#12345", true},
		{"#1234567", true},
		{"#12", true},
		{"red", true},
		{"rgb(0,0,0)", true},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			c, err := ParseColor(tt.input)
			if tt.wantErr {
				if err == nil {
					t.Errorf("ParseColor(%q) expected error, got nil", tt.input)
				}
				if !c.IsZero() {
					t.Errorf("ParseColor(%q) on error: expected zero Color", tt.input)
				}
			} else {
				if err != nil {
					t.Errorf("ParseColor(%q) unexpected error: %v", tt.input, err)
				}
				if c.String() != tt.input {
					t.Errorf("ParseColor(%q).String() = %q, want %q", tt.input, c.String(), tt.input)
				}
				if c.IsZero() {
					t.Errorf("ParseColor(%q) should not be zero", tt.input)
				}
			}
		})
	}
}

func TestColorZeroValue(t *testing.T) {
	var c Color
	if !c.IsZero() {
		t.Error("zero Color should be zero")
	}
	if c.String() != "" {
		t.Errorf("zero Color.String() = %q, want \"\"", c.String())
	}
}
