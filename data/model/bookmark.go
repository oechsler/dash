package model

import "gorm.io/gorm"

type Bookmark struct {
	gorm.Model
	CategoryID  uint     `gorm:"not null;index"`
	Category    Category `gorm:"constraint:OnUpdate:CASCADE,OnDelete:CASCADE"`
	Icon        string   `gorm:"not null;default:''"`
	DisplayName string   `gorm:"not null;default:''"`
	Description *string
	Url         string `gorm:"not null;default:''"`
}

func (b *Bookmark) TableName() string {
	return "bookmarks"
}
