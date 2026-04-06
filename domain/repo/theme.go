package repo

import "context"

// ThemeRecord is the data transfer type exchanged with the ThemeRepository.
type ThemeRecord struct {
	ID          uint
	UserID      string
	DisplayName string
	Primary     string
	Secondary   string
	Tertiary    string
	Deletable   bool
}

type ThemeRepository interface {
	Create(ctx context.Context, record *ThemeRecord) error
	Delete(ctx context.Context, userID string, id uint) error
	DeleteAllByUser(ctx context.Context, userID string) error
	ListByUser(ctx context.Context, userID string) ([]ThemeRecord, error)
	// GetByID returns nil, nil when the theme is not found.
	GetByID(ctx context.Context, userID string, id uint) (*ThemeRecord, error)
}
