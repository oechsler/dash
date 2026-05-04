package command

import (
	"context"
	"errors"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// UserThemeDeleter handles the delete-user-theme command.
type UserThemeDeleter interface {
	Handle(ctx context.Context, userID string, id uint) error
}

type DeleteUserTheme struct {
	Repo        domainrepo.ThemeRepository
	SettingRepo domainrepo.SettingRepository
}

func NewDeleteUserTheme(r domainrepo.ThemeRepository, s domainrepo.SettingRepository) *DeleteUserTheme {
	return &DeleteUserTheme{Repo: r, SettingRepo: s}
}

func (h *DeleteUserTheme) Handle(ctx context.Context, userID string, id uint) error {
	_, err := h.Repo.GetByID(ctx, userID, id)
	if err != nil {
		var nfe *domainerrors.NotFoundError
		if errors.As(err, &nfe) {
			return nil
		}
		return domainerrors.Internal("delete user theme: get by id", err)
	}

	// Prevent deleting the theme that is currently active in the user's settings.
	// The user must switch to another theme (or the synthetic default) first.
	setting, err := h.SettingRepo.GetByUserID(ctx, userID)
	if err != nil {
		var nfe *domainerrors.NotFoundError
		if !errors.As(err, &nfe) {
			return domainerrors.Internal("delete user theme: get settings", err)
		}
	}
	if setting != nil && setting.ThemeID == id {
		return domainerrors.Forbidden("theme is currently active")
	}

	if err := h.Repo.Delete(ctx, userID, id); err != nil {
		return domainerrors.Internal("delete user theme: delete", err)
	}
	return nil
}
