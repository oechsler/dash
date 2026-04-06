package command

import (
	"context"
	"errors"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// UserDataDeleter handles the delete-user-data command.
type UserDataDeleter interface {
	Handle(ctx context.Context, userID string) error
}

type DeleteUserData struct {
	DashboardRepo domainrepo.DashboardRepository
	SettingRepo   domainrepo.SettingRepository
	ThemeRepo     domainrepo.ThemeRepository
}

func NewDeleteUserData(
	dashboardRepo domainrepo.DashboardRepository,
	settingRepo domainrepo.SettingRepository,
	themeRepo domainrepo.ThemeRepository,
) *DeleteUserData {
	return &DeleteUserData{
		DashboardRepo: dashboardRepo,
		SettingRepo:   settingRepo,
		ThemeRepo:     themeRepo,
	}
}

func (h *DeleteUserData) Handle(ctx context.Context, userID string) error {
	// Delete dashboard (cascades to categories and bookmarks via FK)
	dashboard, err := h.DashboardRepo.GetByUserID(ctx, userID)
	if err != nil {
		var nfe *domainerrors.NotFoundError
		if !errors.As(err, &nfe) {
			return domainerrors.Internal("delete user data: get dashboard", err)
		}
	} else {
		if err := h.DashboardRepo.Delete(ctx, dashboard.ID); err != nil {
			return domainerrors.Internal("delete user data: delete dashboard", err)
		}
	}

	// Delete settings
	if err := h.SettingRepo.DeleteByUserID(ctx, userID); err != nil {
		return domainerrors.Internal("delete user data: delete settings", err)
	}

	// Delete all user themes (including non-deletable)
	if err := h.ThemeRepo.DeleteAllByUser(ctx, userID); err != nil {
		return domainerrors.Internal("delete user data: delete themes", err)
	}

	return nil
}
