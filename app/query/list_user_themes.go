package query

import (
	"context"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// UserThemesLister handles the list-user-themes query.
type UserThemesLister interface {
	Handle(ctx context.Context, userID string, activeThemeID uint) ([]domainmodel.Theme, error)
}

type ListUserThemes struct {
	ThemeRepo domainrepo.ThemeRepository
}

func NewListUserThemes(themeRepo domainrepo.ThemeRepository) *ListUserThemes {
	return &ListUserThemes{ThemeRepo: themeRepo}
}

func (h *ListUserThemes) Handle(ctx context.Context, userID string, activeThemeID uint) ([]domainmodel.Theme, error) {
	list, err := h.ThemeRepo.ListByUser(ctx, userID)
	if err != nil {
		return nil, domainerrors.Internal("list user themes", err)
	}

	// The synthetic default is always first; skip any DB rows that are identical
	// to it (old persisted system themes from before the migration).
	def := domainmodel.DefaultTheme()
	def.Deletable = false // default is never deletable regardless of active state
	out := []domainmodel.Theme{def}
	for _, t := range list {
		if domainmodel.IsSyntheticDuplicate(t.DisplayName, t.Primary, t.Secondary, t.Tertiary) {
			continue
		}
		out = append(out, domainmodel.Theme{
			ID:        t.ID,
			Name:      t.DisplayName,
			Primary:   t.Primary,
			Secondary: t.Secondary,
			Tertiary:  t.Tertiary,
			Deletable: t.ID != activeThemeID,
		})
	}

	return out, nil
}
