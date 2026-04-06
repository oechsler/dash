package repo

import "context"

// CategoryRecord is the data transfer type exchanged with the CategoryRepository.
type CategoryRecord struct {
	ID          uint
	DashboardID uint
	DisplayName string
	IsShelved   bool
}

type CategoryRepository interface {
	Upsert(ctx context.Context, record *CategoryRecord) error
	Get(ctx context.Context, id uint) (*CategoryRecord, error)
	ListByDashboardID(ctx context.Context, dashboardID uint) ([]CategoryRecord, error)
	Delete(ctx context.Context, id uint) error
}
