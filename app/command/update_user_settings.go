package command

import (
	"context"
	"errors"
	"time"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"

	"git.at.oechsler.it/samuel/dash/v2/app/validation"
)

// UpdateUserSettingsCmd is the input for updating a user's display settings.
type UpdateUserSettingsCmd struct {
	ThemeID  uint   `validate:"required,gt=0"`
	Language string `validate:"omitempty,oneof=auto en de"`
	Timezone string `validate:"omitempty"`
}

// UserSettingsUpdater handles the UpdateUserSettingsCmd command.
type UserSettingsUpdater interface {
	Handle(ctx context.Context, userId string, in UpdateUserSettingsCmd) error
}

type UpdateUserSettings struct {
	SettingRepo domainrepo.SettingRepository
	ThemeRepo   domainrepo.ThemeRepository
	Validator   validation.Validator
}

func NewUpdateUserSettings(settingRepo domainrepo.SettingRepository, themeRepo domainrepo.ThemeRepository, v validation.Validator) *UpdateUserSettings {
	return &UpdateUserSettings{SettingRepo: settingRepo, ThemeRepo: themeRepo, Validator: v}
}

func (h *UpdateUserSettings) Handle(ctx context.Context, userId string, in UpdateUserSettingsCmd) error {
	if err := h.Validator.Struct(in); err != nil {
		return domainerrors.Validation(validation.ToViolations(err)...)
	}

	existing, err := h.SettingRepo.GetByUserID(ctx, userId)
	if err != nil {
		var nfe *domainerrors.NotFoundError
		if errors.As(err, &nfe) {
			existing = &domainrepo.SettingRecord{UserID: userId}
		} else {
			return domainerrors.Internal("update user settings: get by user", err)
		}
	}

	if in.ThemeID != 0 {
		_, err := h.ThemeRepo.GetByID(ctx, userId, in.ThemeID)
		if err != nil {
			return domainerrors.WrapRepo("update user settings: get theme by id", err)
		}
		existing.ThemeID = in.ThemeID
	}

	if in.Language != "" {
		existing.Language = in.Language
	}

	if in.Timezone != "" {
		if in.Timezone != "auto" {
			if _, err := time.LoadLocation(in.Timezone); err != nil {
				return domainerrors.Validation(domainerrors.Violation{Field: "timezone", Message: "must be a valid IANA timezone"})
			}
		}
		existing.Timezone = in.Timezone
	}

	if err := h.SettingRepo.Upsert(ctx, existing); err != nil {
		return domainerrors.Internal("update user settings: upsert", err)
	}
	return nil
}
