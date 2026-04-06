package repo

import "context"

// DashboardRecord is the data transfer type exchanged with the DashboardRepository.
type DashboardRecord struct {
	ID     uint
	UserID string
}

type DashboardRepository interface {
	Upsert(ctx context.Context, record *DashboardRecord) error
	Get(ctx context.Context, id uint) (*DashboardRecord, error)
	GetByUserID(ctx context.Context, userID string) (*DashboardRecord, error)
	Delete(ctx context.Context, id uint) error
}
