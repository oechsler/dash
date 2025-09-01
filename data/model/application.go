package model

import "gorm.io/gorm"

type Application struct {
	gorm.Model
	Icon            string `gorm:"not null;default:''"`
	DisplayName     string `gorm:"not null;default:''"`
	Description     *string
	Url             string   `gorm:"not null;default:''"`
	VisibleToGroups []string `gorm:"serializer:json;not null;default:'[]'"`
}

func (a *Application) TableName() string {
	return "applications"
}
