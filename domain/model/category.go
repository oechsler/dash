package model

import (
	"errors"
	"strings"
)

type Category struct {
	ID          uint       `json:"id"`
	DisplayName string     `json:"display_name"`
	IsShelved   bool       `json:"is_shelved"`
	Bookmarks   []Bookmark `json:"bookmarks"`
}

// NewCategory creates a Category with a validated display name.
func NewCategory(displayName string) (Category, error) {
	displayName = strings.TrimSpace(displayName)
	if displayName == "" {
		return Category{}, errors.New("category: display name must not be empty")
	}
	return Category{DisplayName: displayName}, nil
}

// Shelve marks the category as shelved (hidden from the main dashboard view).
func (c *Category) Shelve() { c.IsShelved = true }

// Unshelve makes the category visible on the main dashboard again.
func (c *Category) Unshelve() { c.IsShelved = false }

// Rename updates the display name, returning an error if the name is empty.
func (c *Category) Rename(name string) error {
	name = strings.TrimSpace(name)
	if name == "" {
		return errors.New("category: display name must not be empty")
	}
	c.DisplayName = name
	return nil
}
