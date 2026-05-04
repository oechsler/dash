package model

type Setting struct {
	Base
	UserID   string `gorm:"not null;uniqueIndex"`
	User     User   `gorm:"constraint:fk_settings_user,OnDelete:CASCADE"`
	ThemeID  *uint  `gorm:"index"`
	Language string `gorm:"not null;default:'auto'"`
	Timezone string `gorm:"not null;default:'auto'"`
}

func (s *Setting) TableName() string {
	return "settings"
}
