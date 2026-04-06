package model


type Dashboard struct {
	Base
	UserId string `gorm:"uniqueIndex"`
}

func (d *Dashboard) TableName() string {
	return "dashboards"
}
