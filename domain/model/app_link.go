package model

type AppLink struct {
	ID              uint        `json:"id"`
	Icon            Icon        `json:"icon"`
	DisplayName     string      `json:"display_name"`
	Url             BookmarkURL `json:"url"`
	VisibleToGroups []string    `json:"visible_to_groups"`
}
