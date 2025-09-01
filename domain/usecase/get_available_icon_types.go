package usecase

import "context"

// GetAvailableIconTypes exposes which icon type prefixes the domain supports.
// UI should use this to populate choices; data layer simply stores values with prefix (e.g., "mdi:home").
// No stylesheet loading decisions are made here.

type GetAvailableIconTypes struct{}

func NewGetAvailableIconTypes() *GetAvailableIconTypes {
	return &GetAvailableIconTypes{}
}

func (uc *GetAvailableIconTypes) Execute(ctx context.Context) ([]string, error) {
	return []string{"mdi", "spi"}, nil
}
