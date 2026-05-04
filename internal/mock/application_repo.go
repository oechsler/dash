package mock

import (
	"context"

	"github.com/stretchr/testify/mock"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

type ApplicationRepository struct{ mock.Mock }

func (m *ApplicationRepository) Upsert(ctx context.Context, record *domainrepo.ApplicationRecord) error {
	return m.Called(ctx, record).Error(0)
}

func (m *ApplicationRepository) Get(ctx context.Context, id uint) (*domainrepo.ApplicationRecord, error) {
	args := m.Called(ctx, id)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*domainrepo.ApplicationRecord), args.Error(1)
}

func (m *ApplicationRepository) List(ctx context.Context) ([]domainrepo.ApplicationRecord, error) {
	args := m.Called(ctx)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).([]domainrepo.ApplicationRecord), args.Error(1)
}

func (m *ApplicationRepository) Delete(ctx context.Context, id uint) error {
	return m.Called(ctx, id).Error(0)
}
