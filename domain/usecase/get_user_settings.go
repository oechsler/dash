package usecase

import (
	"context"
	"dash/data/model"
	"dash/data/repo"
	domainmodel "dash/domain/model"
)

type GetUserSettings struct {
	SettingRepo          repo.SettingRepo
	ThemeRepo            repo.ThemeRepo
	EnsureDefaultThemeUC *EnsureDefaultTheme
}

func NewGetUserSettings(
	settingRepo repo.SettingRepo,
	themeRepo repo.ThemeRepo,
	ensureDefault *EnsureDefaultTheme,
) *GetUserSettings {
	return &GetUserSettings{
		SettingRepo:          settingRepo,
		ThemeRepo:            themeRepo,
		EnsureDefaultThemeUC: ensureDefault,
	}
}

func (uc *GetUserSettings) Execute(ctx context.Context, userId string) (*domainmodel.Setting, error) {
	s, err := uc.SettingRepo.GetByUserId(ctx, userId)
	if err != nil {
		return nil, Internal("get user settings: get by user", err)
	}
	if s == nil {
		// Use the domain use case to ensure a default theme exists and pick the correct one
		defTheme, err := uc.EnsureDefaultThemeUC.Execute(ctx, userId)
		if err != nil {
			return nil, Internal("get user settings: ensure default theme", err)
		}
		if err := uc.SettingRepo.Upsert(ctx, &model.Setting{UserId: userId, ThemeID: defTheme.ID}); err != nil {
			return nil, Internal("get user settings: upsert setting", err)
		}
		s, err = uc.SettingRepo.GetByUserId(ctx, userId)
		if err != nil {
			return nil, Internal("get user settings: re-fetch", err)
		}
	}
	return &domainmodel.Setting{
		ThemeID: s.ThemeID,
	}, nil
}
