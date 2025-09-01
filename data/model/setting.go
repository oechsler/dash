package model

import "gorm.io/gorm"

type Setting struct {
	gorm.Model
	UserId  string `gorm:"not null;uniqueIndex"`
	ThemeID uint   `gorm:"not null;index"`
}

func (s *Setting) TableName() string {
	return "settings"
}
