package model

import "time"

// Session is the GORM model for the sessions table.
type Session struct {
	ID          string    `gorm:"primaryKey"`
	CreatedAt   time.Time
	UpdatedAt   time.Time
	UserID      string    `gorm:"not null;index"`
	SessionID   string    `gorm:"not null;uniqueIndex"`
	IssuedAt    time.Time
	ExpiresAt   time.Time `gorm:"not null"`
	PinnedUntil    time.Time
	LastAccessedAt time.Time
	LastIP         string
	UserAgent      string
	Sub         string `gorm:"not null"`
	Username    string `gorm:"not null"`
	Email       string `gorm:"not null"`
	FirstName   string
	LastName    string
	DisplayName string
	Picture     string
	ProfileUrl  string
	Groups      string `gorm:"type:text"` // JSON-encoded []string
	IsAdmin     bool
}
