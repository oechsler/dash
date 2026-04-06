package query

import (
	"context"
	"errors"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	"git.at.oechsler.it/samuel/dash/v2/app/command"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// UserSettingsGetter handles the get-user-settings query.
type UserSettingsGetter interface {
	Handle(ctx context.Context, userId string) (*domainmodel.Setting, error)
}

type GetUserSettings struct {
	SettingRepo          domainrepo.SettingRepository
	ThemeRepo            domainrepo.ThemeRepository
	EnsureDefaultThemeUC command.DefaultThemeEnsurer
}

func NewGetUserSettings(
	settingRepo domainrepo.SettingRepository,
	themeRepo domainrepo.ThemeRepository,
	ensureDefault command.DefaultThemeEnsurer,
) *GetUserSettings {
	return &GetUserSettings{
		SettingRepo:          settingRepo,
		ThemeRepo:            themeRepo,
		EnsureDefaultThemeUC: ensureDefault,
	}
}

func (h *GetUserSettings) Handle(ctx context.Context, userId string) (*domainmodel.Setting, error) {
	s, err := h.SettingRepo.GetByUserID(ctx, userId)
	if err != nil {
		var nfe *domainerrors.NotFoundError
		if !errors.As(err, &nfe) {
			return nil, domainerrors.Internal("get user settings: get by user", err)
		}
		// no settings yet, auto-provision
		defTheme, err := h.EnsureDefaultThemeUC.Handle(ctx, userId)
		if err != nil {
			return nil, domainerrors.Internal("get user settings: ensure default theme", err)
		}
		if err := h.SettingRepo.Upsert(ctx, &domainrepo.SettingRecord{UserID: userId, ThemeID: defTheme.ID}); err != nil {
			return nil, domainerrors.Internal("get user settings: upsert setting", err)
		}
		s, err = h.SettingRepo.GetByUserID(ctx, userId)
		if err != nil {
			return nil, domainerrors.Internal("get user settings: re-fetch", err)
		}
	}
	return &domainmodel.Setting{
		ThemeID:  s.ThemeID,
		Language: s.Language,
		Timezone: s.Timezone,
	}, nil
}
