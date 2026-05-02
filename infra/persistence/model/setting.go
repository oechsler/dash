package model


type Setting struct {
	Base
	UserId   string `gorm:"not null;uniqueIndex"`
	ThemeID  uint   `gorm:"not null;index"`
	Language string `gorm:"not null;default:'auto'"`
	Timezone string `gorm:"not null;default:'auto'"`
	User     User   `gorm:"constraint:fk_settings_user,OnDelete:CASCADE"`
	Theme    Theme  `gorm:"constraint:fk_settings_theme,OnDelete:RESTRICT"`
}

func (s *Setting) TableName() string {
	return "settings"
}
