package mock

import (
	"context"

	"github.com/stretchr/testify/mock"
)

type IdpLinkRepository struct{ mock.Mock }

func (m *IdpLinkRepository) ResolveOrCreate(ctx context.Context, issuer, sub string) (string, bool, error) {
	args := m.Called(ctx, issuer, sub)
	return args.String(0), args.Bool(1), args.Error(2)
}

func (m *IdpLinkRepository) DeleteByUserID(ctx context.Context, userID string) error {
	return m.Called(ctx, userID).Error(0)
}
