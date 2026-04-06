package model

type Bookmark struct {
	ID          uint        `json:"id"`
	Icon        Icon        `json:"icon"`
	DisplayName string      `json:"display_name"`
	Url         BookmarkURL `json:"url"`
	CategoryID  uint        `json:"category_id"`
}

// UpdateIcon replaces the bookmark's icon.
func (b *Bookmark) UpdateIcon(icon Icon) { b.Icon = icon }

// Rename sets a new display name for the bookmark.
func (b *Bookmark) Rename(name string) { b.DisplayName = name }

// ChangeURL replaces the bookmark's URL.
func (b *Bookmark) ChangeURL(url BookmarkURL) { b.Url = url }

// MoveTo reassigns the bookmark to a different category.
func (b *Bookmark) MoveTo(categoryID uint) { b.CategoryID = categoryID }
