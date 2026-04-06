package model


type Setting struct {
	Base
	UserId   string `gorm:"not null;uniqueIndex"`
	ThemeID  uint   `gorm:"not null;index"`
	Language string `gorm:"not null;default:'auto'"`
	Timezone string `gorm:"not null;default:'auto'"`
}

func (s *Setting) TableName() string {
	return "settings"
}
