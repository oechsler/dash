package model

type AppLink struct {
	ID              uint     `json:"id"`
	Icon            string   `json:"icon"`
	DisplayName     string   `json:"display_name"`
	Description     *string  `json:"description,omitempty"`
	Url             string   `json:"url"`
	VisibleToGroups []string `json:"visible_to_groups"`
}
