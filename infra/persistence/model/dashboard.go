package model


type Dashboard struct {
	Base
	UserId string `gorm:"uniqueIndex"`
	User   User   `gorm:"constraint:fk_dashboards_user,OnDelete:CASCADE"`
}

func (d *Dashboard) TableName() string {
	return "dashboards"
}
