package mock

import (
	"context"

	"github.com/stretchr/testify/mock"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

type SettingRepository struct{ mock.Mock }

func (m *SettingRepository) Upsert(ctx context.Context, record *domainrepo.SettingRecord) error {
	return m.Called(ctx, record).Error(0)
}

func (m *SettingRepository) GetByUserID(ctx context.Context, userID string) (*domainrepo.SettingRecord, error) {
	args := m.Called(ctx, userID)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*domainrepo.SettingRecord), args.Error(1)
}

func (m *SettingRepository) DeleteByUserID(ctx context.Context, userID string) error {
	return m.Called(ctx, userID).Error(0)
}
