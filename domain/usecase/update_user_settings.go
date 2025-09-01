package usecase

import (
	"context"
	"dash/data/model"
	"dash/data/repo"
	domainmodel "dash/domain/model"
)

type UpdateUserSettings struct {
	SettingRepo repo.SettingRepo
	ThemeRepo   repo.ThemeRepo
}

func NewUpdateUserSettings(settingRepo repo.SettingRepo, themeRepo repo.ThemeRepo) *UpdateUserSettings {
	return &UpdateUserSettings{SettingRepo: settingRepo, ThemeRepo: themeRepo}
}

func (uc *UpdateUserSettings) Execute(ctx context.Context, userId string, updates domainmodel.Setting) (*domainmodel.Setting, error) {
	existing, err := uc.SettingRepo.GetByUserId(ctx, userId)
	if err != nil {
		return nil, err
	}
	if existing == nil {
		existing = &model.Setting{UserId: userId}
	}
	if updates.ThemeID != 0 {
		// validate theme exists for this user
		if _, err := uc.ThemeRepo.GetByID(ctx, userId, updates.ThemeID); err == nil {
			existing.ThemeID = updates.ThemeID
		}
	}
	if updates.Language != "" {
		existing.Language = updates.Language
	}
	if updates.TimeZone != "" {
		existing.TimeZone = updates.TimeZone
	}

	if err := uc.SettingRepo.Upsert(ctx, existing); err != nil {
		return nil, err
	}
	return &domainmodel.Setting{
		ThemeID:  existing.ThemeID,
		Language: existing.Language,
		TimeZone: existing.TimeZone,
	}, nil
}
