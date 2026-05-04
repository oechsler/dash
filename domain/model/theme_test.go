package model

import "testing"

func TestDefaultTheme(t *testing.T) {
	d := DefaultTheme()
	if d.ID != 0 {
		t.Errorf("DefaultTheme().ID = %d, want 0", d.ID)
	}
	if d.Name == "" {
		t.Error("DefaultTheme().Name must not be empty")
	}
	if d.Primary == "" || d.Secondary == "" || d.Tertiary == "" {
		t.Error("DefaultTheme() must have all color fields set")
	}
	if d.Deletable {
		t.Error("DefaultTheme() must not be deletable")
	}
}

func TestIsDefaultThemeID(t *testing.T) {
	if !IsDefaultThemeID(0) {
		t.Error("IsDefaultThemeID(0) should be true")
	}
	if IsDefaultThemeID(1) {
		t.Error("IsDefaultThemeID(1) should be false")
	}
	if IsDefaultThemeID(999) {
		t.Error("IsDefaultThemeID(999) should be false")
	}
}

func TestIsSyntheticDuplicate(t *testing.T) {
	d := DefaultTheme()

	tests := []struct {
		name      string
		primary   string
		secondary string
		tertiary  string
		want      bool
	}{
		// Exact color match
		{"anything", d.Primary, d.Secondary, d.Tertiary, true},
		// Name match, different colors
		{d.Name, "#000000", "#000000", "#000000", true},
		// Partial color match is not enough
		{"other", d.Primary, d.Secondary, "#000000", false},
		// No match at all
		{"My Theme", "#111111", "#222222", "#333333", false},
		// Both match
		{d.Name, d.Primary, d.Secondary, d.Tertiary, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := IsSyntheticDuplicate(tt.name, tt.primary, tt.secondary, tt.tertiary)
			if got != tt.want {
				t.Errorf("IsSyntheticDuplicate(%q, ...) = %v, want %v", tt.name, got, tt.want)
			}
		})
	}
}
