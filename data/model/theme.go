package model

import "gorm.io/gorm"

type Theme struct {
	gorm.Model
	UserId      string `gorm:"not null;index"`
	DisplayName string `gorm:"not null"`
	Primary     string `gorm:"not null"`
	Secondary   string `gorm:"not null"`
	Tertiary    string `gorm:"not null"`
	Deletable   bool   `gorm:"not null;default:false"`
}

func (t *Theme) TableName() string {
	return "themes"
}
