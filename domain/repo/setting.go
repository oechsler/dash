package repo

import "context"

// SettingRecord is the data transfer type exchanged with the SettingRepository.
type SettingRecord struct {
	ID       uint
	UserID   string
	ThemeID  uint
	Language string
	Timezone string
}

type SettingRepository interface {
	Upsert(ctx context.Context, record *SettingRecord) error
	GetByUserID(ctx context.Context, userID string) (*SettingRecord, error)
	DeleteByUserID(ctx context.Context, userID string) error
}
