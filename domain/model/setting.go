package model

type Setting struct {
	ThemeID  uint   `json:"theme_id"`
	Language string `json:"language"`
	Timezone string `json:"timezone"`
}
