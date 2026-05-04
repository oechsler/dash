package model


type Setting struct {
	Base
	UserID   string `gorm:"not null;uniqueIndex"`
	User     User   `gorm:"constraint:fk_settings_user,OnDelete:CASCADE"`
	ThemeID  uint   `gorm:"not null;index"`
	Theme    Theme  `gorm:"constraint:fk_settings_theme,OnDelete:RESTRICT"`
	Language string `gorm:"not null;default:'auto'"`
	Timezone string `gorm:"not null;default:'auto'"`
}

func (s *Setting) TableName() string {
	return "settings"
}
