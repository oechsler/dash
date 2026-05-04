package model

import (
	"testing"
)

func TestParseIcon(t *testing.T) {
	tests := []struct {
		input    string
		wantErr  bool
		wantType string
		wantName string
	}{
		{"mdi:home", false, "mdi", "home"},
		{"mdi:arrow-left", false, "mdi", "arrow-left"},
		{"spi:server", false, "spi", "server"},
		{"spi:some:name", false, "spi", "some:name"}, // SplitN(2) keeps remainder
		{"", true, "", ""},
		{"mdi", true, "", ""},
		{"nocolon", true, "", ""},
		{"unknown:home", true, "", ""},
		{"mdi:", true, "", ""},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			icon, err := ParseIcon(tt.input)
			if tt.wantErr {
				if err == nil {
					t.Errorf("ParseIcon(%q) expected error, got nil", tt.input)
				}
				if !icon.IsZero() {
					t.Errorf("ParseIcon(%q) on error: expected zero Icon", tt.input)
				}
			} else {
				if err != nil {
					t.Errorf("ParseIcon(%q) unexpected error: %v", tt.input, err)
				}
				if icon.Type() != tt.wantType {
					t.Errorf("Type() = %q, want %q", icon.Type(), tt.wantType)
				}
				if icon.Name() != tt.wantName {
					t.Errorf("Name() = %q, want %q", icon.Name(), tt.wantName)
				}
				if icon.IsZero() {
					t.Errorf("ParseIcon(%q) should not be zero", tt.input)
				}
			}
		})
	}
}

func TestNewIcon(t *testing.T) {
	tests := []struct {
		iconType string
		name     string
		wantErr  bool
	}{
		{"mdi", "home", false},
		{"spi", "server", false},
		{"", "home", true},
		{"mdi", "", true},
		{"unknown", "home", true},
		{"svg", "anything", true},
	}

	for _, tt := range tests {
		t.Run(tt.iconType+":"+tt.name, func(t *testing.T) {
			icon, err := NewIcon(tt.iconType, tt.name)
			if tt.wantErr {
				if err == nil {
					t.Errorf("NewIcon(%q, %q) expected error, got nil", tt.iconType, tt.name)
				}
			} else {
				if err != nil {
					t.Errorf("NewIcon(%q, %q) unexpected error: %v", tt.iconType, tt.name, err)
				}
				if icon.Type() != tt.iconType {
					t.Errorf("Type() = %q, want %q", icon.Type(), tt.iconType)
				}
				if icon.Name() != tt.name {
					t.Errorf("Name() = %q, want %q", icon.Name(), tt.name)
				}
				if icon.String() != tt.iconType+":"+tt.name {
					t.Errorf("String() = %q, want %q", icon.String(), tt.iconType+":"+tt.name)
				}
			}
		})
	}
}

func TestIconZeroValue(t *testing.T) {
	var i Icon
	if !i.IsZero() {
		t.Error("zero Icon should be zero")
	}
}

func TestKnownIconTypes(t *testing.T) {
	types := KnownIconTypes()
	if len(types) == 0 {
		t.Error("KnownIconTypes() must return at least one type")
	}
	seen := map[string]bool{}
	for _, tp := range types {
		if tp == "" {
			t.Error("KnownIconTypes() must not contain empty string")
		}
		seen[tp] = true
	}
	if !seen["mdi"] {
		t.Error("KnownIconTypes() must include \"mdi\"")
	}
	if !seen["spi"] {
		t.Error("KnownIconTypes() must include \"spi\"")
	}
}
