package usecase

import (
	"context"
	datamodel "dash/data/model"
	"dash/data/repo"
	dom "dash/domain/model"
)

type EnsureDefaultTheme struct {
	Repo repo.ThemeRepo
}

func NewEnsureDefaultTheme(r repo.ThemeRepo) *EnsureDefaultTheme {
	return &EnsureDefaultTheme{
		Repo: r,
	}
}

// Execute ensures there is at least one theme for the user and returns a stable default.
// Business rule (domain-owned):
// - If user has no themes, create a non-deletable default theme.
// - Else, prefer an existing non-deletable theme; if none, pick the oldest (smallest ID).
func (uc *EnsureDefaultTheme) Execute(ctx context.Context, userID string) (*dom.Theme, error) {
	list, err := uc.Repo.ListByUser(ctx, userID)
	if err != nil {
		return nil, Internal("ensure default theme: list", err)
	}
	if len(list) == 0 {
		def := &datamodel.Theme{
			UserId:      userID,
			DisplayName: "Catppuccin Mocha (Mauve)",
			Primary:     "#1e1e2e", // Base
			Secondary:   "#cdd6f4", // Text
			Tertiary:    "#cba6f7", // Mauve
			Deletable:   false,
		}
		if err := uc.Repo.Create(ctx, def); err != nil {
			return nil, Internal("ensure default theme: create", err)
		}
		return &dom.Theme{
			ID:        def.ID,
			Name:      def.DisplayName,
			Primary:   def.Primary,
			Secondary: def.Secondary,
			Tertiary:  def.Tertiary,
			Deletable: def.Deletable,
		}, nil
	}

	var chosen *datamodel.Theme
	for i := range list {
		if !list[i].Deletable {
			chosen = &list[i]
			break
		}
	}
	if chosen == nil {
		minIdx := 0
		for i := 1; i < len(list); i++ {
			if list[i].ID < list[minIdx].ID {
				minIdx = i
			}
		}
		chosen = &list[minIdx]
	}
	return &dom.Theme{
		ID:        chosen.ID,
		Name:      chosen.DisplayName,
		Primary:   chosen.Primary,
		Secondary: chosen.Secondary,
		Tertiary:  chosen.Tertiary,
		Deletable: chosen.Deletable,
	}, nil
}
