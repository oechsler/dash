package model

type Category struct {
	ID          uint       `json:"id"`
	DisplayName string     `json:"display_name"`
	IsShelved   bool       `json:"is_shledved"`
	Bookmarks   []Bookmark `json:"bookmarks"`
}
