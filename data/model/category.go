package model

import "gorm.io/gorm"

type Category struct {
	gorm.Model
	DashboardID uint      `gorm:"not null;index"`
	Dashboard   Dashboard `gorm:"constraint:OnUpdate:CASCADE,OnDelete:CASCADE"`
	DisplayName string    `gorm:"not null:"`
	IsShelved   bool      `gorm:"not null;default:false"`
}

func (c *Category) TableName() string {
	return "categories"
}
