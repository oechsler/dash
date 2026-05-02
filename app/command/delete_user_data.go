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
	SessionRepo   domainrepo.SessionRepository
	IdpLinkRepo   domainrepo.IdpLinkRepository
}

func NewDeleteUserData(
	dashboardRepo domainrepo.DashboardRepository,
	settingRepo domainrepo.SettingRepository,
	themeRepo domainrepo.ThemeRepository,
	sessionRepo domainrepo.SessionRepository,
	idpLinkRepo domainrepo.IdpLinkRepository,
) *DeleteUserData {
	return &DeleteUserData{
		DashboardRepo: dashboardRepo,
		SettingRepo:   settingRepo,
		ThemeRepo:     themeRepo,
		SessionRepo:   sessionRepo,
		IdpLinkRepo:   idpLinkRepo,
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

	// Delete all sessions
	if err := h.SessionRepo.DeleteByUserID(ctx, userID); err != nil {
		return domainerrors.Internal("delete user data: delete sessions", err)
	}

	// Delete all IdP links so the user can register fresh from any IdP
	if err := h.IdpLinkRepo.DeleteByUserID(ctx, userID); err != nil {
		return domainerrors.Internal("delete user data: delete idp links", err)
	}

	return nil
}
