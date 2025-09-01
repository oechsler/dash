package model

import "gorm.io/gorm"

type Dashboard struct {
	gorm.Model
	UserId string `gorm:"uniqueIndex"`
}

func (d *Dashboard) TableName() string {
	return "dashboards"
}
