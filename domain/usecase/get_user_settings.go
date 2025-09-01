package usecase

import (
	"context"
	"dash/data/model"
	"dash/data/repo"
	domainmodel "dash/domain/model"
)

type GetUserSettings struct {
	SettingRepo repo.SettingRepo
	ThemeRepo   repo.ThemeRepo
}

func NewGetUserSettings(settingRepo repo.SettingRepo, themeRepo repo.ThemeRepo) *GetUserSettings {
	return &GetUserSettings{SettingRepo: settingRepo, ThemeRepo: themeRepo}
}

func (uc *GetUserSettings) Execute(ctx context.Context, userId string) (*domainmodel.Setting, error) {
	s, err := uc.SettingRepo.GetByUserId(ctx, userId)
	if err != nil {
		return nil, err
	}
	if s == nil {
		// ensure a default theme exists and set it
		def, err := uc.ThemeRepo.EnsureDefault(ctx, userId)
		if err != nil {
			return nil, err
		}
		if err := uc.SettingRepo.Upsert(ctx, &model.Setting{
			UserId:   userId,
			ThemeID:  def.ID,
			Language: "en",
			TimeZone: "Local",
		}); err != nil {
			return nil, err
		}
		s, err = uc.SettingRepo.GetByUserId(ctx, userId)
		if err != nil {
			return nil, err
		}
	}
	return &domainmodel.Setting{
		ThemeID:  s.ThemeID,
		Language: s.Language,
		TimeZone: s.TimeZone,
	}, nil
}
