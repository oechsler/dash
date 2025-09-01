package model

import "gorm.io/gorm"

type Application struct {
	gorm.Model
	Icon            string   `gorm:"not null"`
	DisplayName     string   `gorm:"not null"`
	Url             string   `gorm:"not null"`
	VisibleToGroups []string `gorm:"serializer:json;not null;default:'[]'"`
}

func (a *Application) TableName() string {
	return "applications"
}
