package mock

import (
	"context"

	"github.com/stretchr/testify/mock"
)

type UserIDMigrationRepository struct{ mock.Mock }

func (m *UserIDMigrationRepository) MigrateUserID(ctx context.Context, oldID, newID string) error {
	return m.Called(ctx, oldID, newID).Error(0)
}
