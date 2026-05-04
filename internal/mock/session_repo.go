package mock

import (
	"context"
	"time"

	"github.com/stretchr/testify/mock"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

type SessionRepository struct{ mock.Mock }

func (m *SessionRepository) Create(ctx context.Context, record *domainrepo.SessionRecord) error {
	return m.Called(ctx, record).Error(0)
}

func (m *SessionRepository) Pin(ctx context.Context, sessionID, userID string, pinnedUntil time.Time) error {
	return m.Called(ctx, sessionID, userID, pinnedUntil).Error(0)
}

func (m *SessionRepository) Unpin(ctx context.Context, recordID, userID string) error {
	return m.Called(ctx, recordID, userID).Error(0)
}

func (m *SessionRepository) Touch(ctx context.Context, sessionID, lastIP, userAgent string) (*domainrepo.SessionRecord, error) {
	args := m.Called(ctx, sessionID, lastIP, userAgent)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*domainrepo.SessionRecord), args.Error(1)
}

func (m *SessionRepository) ListByUserID(ctx context.Context, userID string) ([]*domainrepo.SessionRecord, error) {
	args := m.Called(ctx, userID)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).([]*domainrepo.SessionRecord), args.Error(1)
}

func (m *SessionRepository) DeleteByID(ctx context.Context, recordID, userID string) error {
	return m.Called(ctx, recordID, userID).Error(0)
}

func (m *SessionRepository) DeleteBySessionID(ctx context.Context, sessionID string) error {
	return m.Called(ctx, sessionID).Error(0)
}

func (m *SessionRepository) DeleteByUserID(ctx context.Context, userID string) error {
	return m.Called(ctx, userID).Error(0)
}

func (m *SessionRepository) RefreshBySessionID(ctx context.Context, record *domainrepo.SessionRecord) error {
	return m.Called(ctx, record).Error(0)
}

func (m *SessionRepository) DeleteExpired(ctx context.Context) error {
	return m.Called(ctx).Error(0)
}
