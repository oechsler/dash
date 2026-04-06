package repo

import "context"

// ApplicationRecord is the data transfer type exchanged with the ApplicationRepository.
type ApplicationRecord struct {
	ID              uint
	Icon            string
	DisplayName     string
	Url             string
	VisibleToGroups []string
}

type ApplicationRepository interface {
	Upsert(ctx context.Context, record *ApplicationRecord) error
	Get(ctx context.Context, id uint) (*ApplicationRecord, error)
	List(ctx context.Context) ([]ApplicationRecord, error)
	Delete(ctx context.Context, id uint) error
}
