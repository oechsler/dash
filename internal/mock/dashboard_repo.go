package mock

import (
	"context"

	"github.com/stretchr/testify/mock"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

type DashboardRepository struct{ mock.Mock }

func (m *DashboardRepository) Upsert(ctx context.Context, record *domainrepo.DashboardRecord) error {
	return m.Called(ctx, record).Error(0)
}

func (m *DashboardRepository) Get(ctx context.Context, id uint) (*domainrepo.DashboardRecord, error) {
	args := m.Called(ctx, id)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*domainrepo.DashboardRecord), args.Error(1)
}

func (m *DashboardRepository) GetByUserID(ctx context.Context, userID string) (*domainrepo.DashboardRecord, error) {
	args := m.Called(ctx, userID)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*domainrepo.DashboardRecord), args.Error(1)
}

func (m *DashboardRepository) Delete(ctx context.Context, id uint) error {
	return m.Called(ctx, id).Error(0)
}
