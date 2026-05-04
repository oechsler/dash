package mock

import (
	"context"

	"github.com/stretchr/testify/mock"
)

type UserRepository struct{ mock.Mock }

func (m *UserRepository) DeleteByID(ctx context.Context, id string) error {
	return m.Called(ctx, id).Error(0)
}
