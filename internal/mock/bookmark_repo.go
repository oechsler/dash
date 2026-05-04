package mock

import (
	"context"

	"github.com/stretchr/testify/mock"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

type BookmarkRepository struct{ mock.Mock }

func (m *BookmarkRepository) Upsert(ctx context.Context, record *domainrepo.BookmarkRecord) error {
	return m.Called(ctx, record).Error(0)
}

func (m *BookmarkRepository) Get(ctx context.Context, id uint) (*domainrepo.BookmarkRecord, error) {
	args := m.Called(ctx, id)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*domainrepo.BookmarkRecord), args.Error(1)
}

func (m *BookmarkRepository) ListByCategoryIDs(ctx context.Context, categoryIDs []uint) ([]domainrepo.BookmarkRecord, error) {
	args := m.Called(ctx, categoryIDs)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).([]domainrepo.BookmarkRecord), args.Error(1)
}

func (m *BookmarkRepository) Delete(ctx context.Context, id uint) error {
	return m.Called(ctx, id).Error(0)
}
