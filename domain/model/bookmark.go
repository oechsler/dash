package model

type Bookmark struct {
	ID          uint   `json:"id"`
	Icon        string `json:"icon"`
	DisplayName string `json:"display_name"`
	Url         string `json:"url"`
	CategoryID  uint   `json:"category_id"`
}
