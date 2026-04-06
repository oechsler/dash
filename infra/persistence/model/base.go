package model

import "time"

// Base replaces gorm.Model without the DeletedAt field,
// so GORM performs hard deletes instead of soft deletes.
type Base struct {
	ID        uint      `gorm:"primaryKey;autoIncrement"`
	CreatedAt time.Time
	UpdatedAt time.Time
}
