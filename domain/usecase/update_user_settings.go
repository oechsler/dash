package usecase

import (
	"context"
	"dash/data/model"
	"dash/data/repo"
	"dash/domain/validation"
	"errors"

	"gorm.io/gorm"
)

type UpdateUserSettingsInput struct {
	ThemeID uint `validate:"required,gt=0"`
}

type UpdateUserSettings struct {
	SettingRepo repo.SettingRepo
	ThemeRepo   repo.ThemeRepo
	Validator   validation.Validator
}

func NewUpdateUserSettings(settingRepo repo.SettingRepo, themeRepo repo.ThemeRepo, v validation.Validator) *UpdateUserSettings {
	return &UpdateUserSettings{SettingRepo: settingRepo, ThemeRepo: themeRepo, Validator: v}
}

func (uc *UpdateUserSettings) Execute(ctx context.Context, userId string, in UpdateUserSettingsInput) error {
	if err := uc.Validator.Struct(in); err != nil {
		return Validation(err)
	}

	existing, err := uc.SettingRepo.GetByUserId(ctx, userId)
	if err != nil {
		return Internal("update user settings: get by user", err)
	}
	if existing == nil {
		existing = &model.Setting{UserId: userId}
	}
	if in.ThemeID != 0 {
		// validate theme exists for this user
		if _, err := uc.ThemeRepo.GetByID(ctx, userId, in.ThemeID); err != nil {
			if errors.Is(err, gorm.ErrRecordNotFound) {
				return ErrThemeNotFound
			}
			return Internal("update user settings: get theme by id", err)
		}
		existing.ThemeID = in.ThemeID
	}

	if err := uc.SettingRepo.Upsert(ctx, existing); err != nil {
		return Internal("update user settings: upsert", err)
	}
	return nil
}
