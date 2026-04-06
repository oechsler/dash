package model

import (
	"fmt"
	"net/url"
)

// BookmarkURL is a validated absolute URL for use in bookmarks and applications.
// The zero value is not a valid BookmarkURL; use ParseBookmarkURL to construct one.
type BookmarkURL struct {
	value string
}

// ParseBookmarkURL validates and wraps a URL string.
func ParseBookmarkURL(raw string) (BookmarkURL, error) {
	if raw == "" {
		return BookmarkURL{}, fmt.Errorf("bookmark url: must not be empty")
	}
	u, err := url.ParseRequestURI(raw)
	if err != nil || u.Host == "" {
		return BookmarkURL{}, fmt.Errorf("bookmark url: %q is not a valid absolute URL", raw)
	}
	return BookmarkURL{value: raw}, nil
}

// String returns the raw URL string.
func (u BookmarkURL) String() string { return u.value }

// Host returns the host (and port if present) of the URL.
func (u BookmarkURL) Host() string {
	parsed, _ := url.Parse(u.value)
	return parsed.Host
}

func (u BookmarkURL) IsZero() bool { return u.value == "" }
