package usecase

import (
	"context"
	"dash/data/repo"
	dom "dash/domain/model"
)

type ListUserThemes struct {
	ThemeRepo   repo.ThemeRepo
	SettingRepo repo.SettingRepo
}

func NewListUserThemes(
	themeRepo repo.ThemeRepo,
	settingRepo repo.SettingRepo,
) *ListUserThemes {
	return &ListUserThemes{
		ThemeRepo:   themeRepo,
		SettingRepo: settingRepo,
	}
}

func (uc *ListUserThemes) Execute(ctx context.Context, userID string) ([]dom.Theme, error) {
	list, err := uc.ThemeRepo.ListByUser(ctx, userID)
	if err != nil {
		return nil, Internal("list user themes", err)
	}

	var activeID uint
	if s, err := uc.SettingRepo.GetByUserId(ctx, userID); err == nil && s != nil {
		activeID = s.ThemeID
	}

	out := make([]dom.Theme, 0, len(list))
	for _, t := range list {
		deletable := t.Deletable
		if activeID != 0 && t.ID == activeID {
			deletable = false
		}
		out = append(out, dom.Theme{
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
