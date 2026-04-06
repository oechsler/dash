package query

import (
	"context"

	"git.at.oechsler.it/samuel/dash/v2/domain/model"
)

// AvailableIconTypesGetter handles the get-available-icon-types query.
type AvailableIconTypesGetter interface {
	Handle(ctx context.Context) ([]string, error)
}

// GetAvailableIconTypes exposes which icon type prefixes the domain supports.
// UI uses this to populate choices; data layer stores values with prefix (e.g., "mdi:home").
type GetAvailableIconTypes struct{}

func NewGetAvailableIconTypes() *GetAvailableIconTypes {
	return &GetAvailableIconTypes{}
}

func (h *GetAvailableIconTypes) Handle(ctx context.Context) ([]string, error) {
	return model.KnownIconTypes(), nil
}
