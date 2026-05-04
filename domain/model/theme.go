package model

type Theme struct {
	ID        uint   `json:"id"`
	Name      string `json:"name"`
	Primary   string `json:"primary"`
	Secondary string `json:"secondary"`
	Tertiary  string `json:"tertiary"`
	Deletable bool   `json:"deletable"`
}

// DefaultTheme returns the built-in synthetic theme (Catppuccin Mocha – Mauve).
// It is never persisted; ID 0 is its sentinel value.
func DefaultTheme() Theme {
	return Theme{
		ID:        0,
		Name:      "Catppuccin Mocha (Mauve)",
		Primary:   "#1e1e2e",
		Secondary: "#cdd6f4",
		Tertiary:  "#cba6f7",
		Deletable: false,
	}
}

// IsDefaultThemeID reports whether id refers to the synthetic default theme.
func IsDefaultThemeID(id uint) bool { return id == DefaultTheme().ID }

// IsSyntheticDuplicate reports whether a theme matches the synthetic default
// well enough to be treated as one. A match on either the color triple OR the
// display name is sufficient — this covers the old persisted system theme
// regardless of whether the user renamed it or tweaked a color.
func IsSyntheticDuplicate(name, primary, secondary, tertiary string) bool {
	d := DefaultTheme()
	colorsMatch := primary == d.Primary && secondary == d.Secondary && tertiary == d.Tertiary
	return colorsMatch || name == d.Name
}
