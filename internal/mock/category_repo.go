package mock

import (
	"context"

	"github.com/stretchr/testify/mock"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

type CategoryRepository struct{ mock.Mock }

func (m *CategoryRepository) Upsert(ctx context.Context, record *domainrepo.CategoryRecord) error {
	return m.Called(ctx, record).Error(0)
}

func (m *CategoryRepository) Get(ctx context.Context, id uint) (*domainrepo.CategoryRecord, error) {
	args := m.Called(ctx, id)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*domainrepo.CategoryRecord), args.Error(1)
}

func (m *CategoryRepository) ListByDashboardID(ctx context.Context, dashboardID uint) ([]domainrepo.CategoryRecord, error) {
	args := m.Called(ctx, dashboardID)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).([]domainrepo.CategoryRecord), args.Error(1)
}

func (m *CategoryRepository) Delete(ctx context.Context, id uint) error {
	return m.Called(ctx, id).Error(0)
}
