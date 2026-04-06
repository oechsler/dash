package service

import "github.com/oechsler-it/dash/domain/repo"

// DefaultTheme selects the user's default theme from a list according to domain rules:
//  1. If list is empty, returns nil — caller must create a default first.
//  2. Prefer a non-deletable (system) theme.
//  3. Fallback: the theme with the smallest ID (oldest).
func DefaultTheme(themes []repo.ThemeRecord) *repo.ThemeRecord {
	if len(themes) == 0 {
		return nil
	}
	for i := range themes {
		if !themes[i].Deletable {
			return &themes[i]
		}
	}
	// fallback: oldest (smallest ID)
	min := 0
	for i := 1; i < len(themes); i++ {
		if themes[i].ID < themes[min].ID {
			min = i
		}
	}
	return &themes[min]
}
