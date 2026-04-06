package model

import (
	"fmt"
	"regexp"
)

var hexColorPattern = regexp.MustCompile(`^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$`)

// Color is a validated CSS hex color string (e.g. "#fff" or "#cba6f7").
// The zero value is not a valid Color; use ParseColor to construct one.
type Color struct {
	value string
}

// ParseColor validates and wraps a CSS hex color string.
func ParseColor(s string) (Color, error) {
	if !hexColorPattern.MatchString(s) {
		return Color{}, fmt.Errorf("color: %q is not a valid hex color (e.g. #fff or #ffffff)", s)
	}
	return Color{value: s}, nil
}

func (c Color) String() string { return c.value }
func (c Color) IsZero() bool   { return c.value == "" }
