package model


type Theme struct {
	Base
	UserID      string `gorm:"not null;index"`
	User        User   `gorm:"constraint:fk_themes_user,OnDelete:CASCADE"`
	DisplayName string `gorm:"not null"`
	Primary     string `gorm:"not null"`
	Secondary   string `gorm:"not null"`
	Tertiary    string `gorm:"not null"`
	Deletable   bool   `gorm:"not null;default:false"`
}

func (t *Theme) TableName() string {
	return "themes"
}
