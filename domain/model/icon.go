package model

import (
	"fmt"
	"strings"
)

var knownIconTypes = []string{"mdi", "spi"}

// Icon represents a namespaced icon reference in "type:name" format (e.g. "mdi:home").
// The zero value is not a valid Icon; use ParseIcon or NewIcon to construct one.
type Icon struct {
	iconType string
	name     string
}

// ParseIcon parses a "type:name" string into an Icon.
func ParseIcon(raw string) (Icon, error) {
	parts := strings.SplitN(raw, ":", 2)
	if len(parts) != 2 {
		return Icon{}, fmt.Errorf("icon: invalid format %q, expected \"type:name\"", raw)
	}
	return NewIcon(parts[0], parts[1])
}

// NewIcon constructs an Icon from a type and name, validating both.
func NewIcon(iconType, name string) (Icon, error) {
	if iconType == "" {
		return Icon{}, fmt.Errorf("icon: type must not be empty")
	}
	if name == "" {
		return Icon{}, fmt.Errorf("icon: name must not be empty")
	}
	valid := false
	for _, t := range knownIconTypes {
		if t == iconType {
			valid = true
			break
		}
	}
	if !valid {
		return Icon{}, fmt.Errorf("icon: unknown type %q, must be one of %v", iconType, knownIconTypes)
	}
	return Icon{iconType: iconType, name: name}, nil
}

func (i Icon) Type() string   { return i.iconType }
func (i Icon) Name() string   { return i.name }
func (i Icon) String() string { return i.iconType + ":" + i.name }
func (i Icon) IsZero() bool   { return i.iconType == "" }

// KnownIconTypes returns the list of valid icon type prefixes.
func KnownIconTypes() []string { return knownIconTypes }
