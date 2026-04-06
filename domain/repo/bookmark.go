package repo

import "context"

// BookmarkRecord is the data transfer type exchanged with the BookmarkRepository.
type BookmarkRecord struct {
	ID          uint
	CategoryID  uint
	Icon        string
	DisplayName string
	Url         string
}

type BookmarkRepository interface {
	Upsert(ctx context.Context, record *BookmarkRecord) error
	Get(ctx context.Context, id uint) (*BookmarkRecord, error)
	ListByCategoryIDs(ctx context.Context, categoryIDs []uint) ([]BookmarkRecord, error)
	Delete(ctx context.Context, id uint) error
}
