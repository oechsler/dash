package model

import "gorm.io/gorm"

type Setting struct {
	gorm.Model
	UserId   string `gorm:"not null;uniqueIndex"`
	ThemeID  uint   `gorm:"not null"`
	Language string `gorm:"not null;default:en"`
	TimeZone string `gorm:"not null;default:Local"`
}
