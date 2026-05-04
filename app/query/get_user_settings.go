package query

import (
	"context"
	"errors"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// UserSettingsGetter handles the get-user-settings query.
type UserSettingsGetter interface {
	Handle(ctx context.Context, userID string) (*domainmodel.Setting, error)
}

type GetUserSettings struct {
	SettingRepo domainrepo.SettingRepository
	ThemeRepo   domainrepo.ThemeRepository
}

func NewGetUserSettings(
	settingRepo domainrepo.SettingRepository,
	themeRepo domainrepo.ThemeRepository,
) *GetUserSettings {
	return &GetUserSettings{
		SettingRepo: settingRepo,
		ThemeRepo:   themeRepo,
	}
}

func (h *GetUserSettings) Handle(ctx context.Context, userID string) (*domainmodel.Setting, error) {
	s, err := h.SettingRepo.GetByUserID(ctx, userID)
	if err != nil {
		var nfe *domainerrors.NotFoundError
		if !errors.As(err, &nfe) {
			return nil, domainerrors.Internal("get user settings: get by user", err)
		}
		// No settings yet — provision with the synthetic default (ThemeID=0).
		if err := h.SettingRepo.Upsert(ctx, &domainrepo.SettingRecord{UserID: userID}); err != nil {
			return nil, domainerrors.Internal("get user settings: upsert setting", err)
		}
		s, err = h.SettingRepo.GetByUserID(ctx, userID)
		if err != nil {
			return nil, domainerrors.Internal("get user settings: re-fetch", err)
		}
	}

	// Normalize: if the stored theme is a persisted copy of the synthetic
	// default, treat it as the synthetic so the UI stays consistent.
	themeID := s.ThemeID
	if !domainmodel.IsDefaultThemeID(themeID) {
		if t, err := h.ThemeRepo.GetByID(ctx, userID, themeID); err == nil {
			if domainmodel.IsSyntheticDuplicate(t.DisplayName, t.Primary, t.Secondary, t.Tertiary) {
				themeID = domainmodel.DefaultTheme().ID
			}
		} else {
			// Theme not found (deleted?) — fall back to synthetic default.
			themeID = domainmodel.DefaultTheme().ID
		}
	}

	return &domainmodel.Setting{
		ThemeID:  themeID,
		Language: s.Language,
		Timezone: s.Timezone,
	}, nil
}
