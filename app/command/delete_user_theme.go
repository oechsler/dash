package command

import (
	"context"
	"errors"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainrepo "github.com/oechsler-it/dash/domain/repo"
)

// UserThemeDeleter handles the delete-user-theme command.
type UserThemeDeleter interface {
	Handle(ctx context.Context, userID string, id uint) error
}

type DeleteUserTheme struct {
	Repo domainrepo.ThemeRepository
}

func NewDeleteUserTheme(r domainrepo.ThemeRepository) *DeleteUserTheme {
	return &DeleteUserTheme{Repo: r}
}

func (h *DeleteUserTheme) Handle(ctx context.Context, userID string, id uint) error {
	list, err := h.Repo.ListByUser(ctx, userID)
	if err != nil {
		return domainerrors.Internal("delete user theme: list", err)
	}
	if len(list) <= 1 {
		return nil
	}

	t, err := h.Repo.GetByID(ctx, userID, id)
	if err != nil {
		var nfe *domainerrors.NotFoundError
		if errors.As(err, &nfe) {
			return nil // theme doesn't exist, silently ignore
		}
		return domainerrors.Internal("delete user theme: get by id", err)
	}
	if !t.Deletable {
		return nil
	}
	if err := h.Repo.Delete(ctx, userID, id); err != nil {
		return domainerrors.Internal("delete user theme: delete", err)
	}
	return nil
}
