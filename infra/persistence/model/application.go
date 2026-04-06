package model


type Application struct {
	Base
	Icon            string   `gorm:"not null"`
	DisplayName     string   `gorm:"not null"`
	Url             string   `gorm:"not null"`
	VisibleToGroups []string `gorm:"serializer:json;not null;default:'[]'"`
}

func (a *Application) TableName() string {
	return "applications"
}
