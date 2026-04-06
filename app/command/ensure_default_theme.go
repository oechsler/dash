package command

import (
	"context"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	"git.at.oechsler.it/samuel/dash/v2/domain/service"
)

// DefaultThemeEnsurer ensures at least one theme exists for the user and returns a stable default.
type DefaultThemeEnsurer interface {
	Handle(ctx context.Context, userID string) (*domainmodel.Theme, error)
}

type EnsureDefaultTheme struct {
	Repo domainrepo.ThemeRepository
}

func NewEnsureDefaultTheme(r domainrepo.ThemeRepository) *EnsureDefaultTheme {
	return &EnsureDefaultTheme{Repo: r}
}

// Handle ensures there is at least one theme for the user and returns a stable default.
// The selection logic (prefer non-deletable; fallback to oldest) lives in service.DefaultTheme.
func (h *EnsureDefaultTheme) Handle(ctx context.Context, userID string) (*domainmodel.Theme, error) {
	list, err := h.Repo.ListByUser(ctx, userID)
	if err != nil {
		return nil, domainerrors.Internal("ensure default theme: list", err)
	}

	if len(list) == 0 {
		def := &domainrepo.ThemeRecord{
			UserID:      userID,
			DisplayName: "Catppuccin Mocha (Mauve)",
			Primary:     "#1e1e2e", // Base
			Secondary:   "#cdd6f4", // Text
			Tertiary:    "#cba6f7", // Mauve
			Deletable:   false,
		}
		if err := h.Repo.Create(ctx, def); err != nil {
			return nil, domainerrors.Internal("ensure default theme: create", err)
		}
		return &domainmodel.Theme{
			ID:        def.ID,
			Name:      def.DisplayName,
			Primary:   def.Primary,
			Secondary: def.Secondary,
			Tertiary:  def.Tertiary,
			Deletable: def.Deletable,
		}, nil
	}

	chosen := service.DefaultTheme(list)
	return &domainmodel.Theme{
		ID:        chosen.ID,
		Name:      chosen.DisplayName,
		Primary:   chosen.Primary,
		Secondary: chosen.Secondary,
		Tertiary:  chosen.Tertiary,
		Deletable: chosen.Deletable,
	}, nil
}
