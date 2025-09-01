package model

import "gorm.io/gorm"

type Theme struct {
	gorm.Model
	UserId    string `gorm:"not null;index:idx_theme_userid"`
	Name      string `gorm:"not null"`
	Primary   string `gorm:"not null"`
	Secondary string `gorm:"not null"`
	Tertiary  string `gorm:"not null"`
	Deletable bool   `gorm:"not null;default:true"`
}
