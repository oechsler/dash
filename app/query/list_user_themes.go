package query

import (
	"context"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// UserThemesLister handles the list-user-themes query.
type UserThemesLister interface {
	Handle(ctx context.Context, userID string) ([]domainmodel.Theme, error)
}

type ListUserThemes struct {
	ThemeRepo   domainrepo.ThemeRepository
	SettingRepo domainrepo.SettingRepository
}

func NewListUserThemes(
	themeRepo domainrepo.ThemeRepository,
	settingRepo domainrepo.SettingRepository,
) *ListUserThemes {
	return &ListUserThemes{
		ThemeRepo:   themeRepo,
		SettingRepo: settingRepo,
	}
}

func (h *ListUserThemes) Handle(ctx context.Context, userID string) ([]domainmodel.Theme, error) {
	list, err := h.ThemeRepo.ListByUser(ctx, userID)
	if err != nil {
		return nil, domainerrors.Internal("list user themes", err)
	}

	var activeID uint
	if s, err := h.SettingRepo.GetByUserID(ctx, userID); err == nil && s != nil {
		activeID = s.ThemeID
	}

	out := make([]domainmodel.Theme, 0, len(list))
	for _, t := range list {
		deletable := t.Deletable
		if activeID != 0 && t.ID == activeID {
			deletable = false
		}
		out = append(out, domainmodel.Theme{
			ID:        t.ID,
			Name:      t.DisplayName,
			Primary:   t.Primary,
			Secondary: t.Secondary,
			Tertiary:  t.Tertiary,
			Deletable: deletable,
		})
	}

	return out, nil
}
