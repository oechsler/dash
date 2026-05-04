package mock

import (
	"context"

	"github.com/stretchr/testify/mock"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

type ThemeRepository struct{ mock.Mock }

func (m *ThemeRepository) Create(ctx context.Context, record *domainrepo.ThemeRecord) error {
	return m.Called(ctx, record).Error(0)
}

func (m *ThemeRepository) Delete(ctx context.Context, userID string, id uint) error {
	return m.Called(ctx, userID, id).Error(0)
}

func (m *ThemeRepository) DeleteAllByUser(ctx context.Context, userID string) error {
	return m.Called(ctx, userID).Error(0)
}

func (m *ThemeRepository) ListByUser(ctx context.Context, userID string) ([]domainrepo.ThemeRecord, error) {
	args := m.Called(ctx, userID)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).([]domainrepo.ThemeRecord), args.Error(1)
}

func (m *ThemeRepository) GetByID(ctx context.Context, userID string, id uint) (*domainrepo.ThemeRecord, error) {
	args := m.Called(ctx, userID, id)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*domainrepo.ThemeRecord), args.Error(1)
}
