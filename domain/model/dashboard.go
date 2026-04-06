package model

type Dashboard struct {
	Applications []AppLink  `json:"applications"`
	Categories   []Category `json:"categories"`
}
