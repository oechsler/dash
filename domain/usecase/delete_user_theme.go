package usecase

import (
	"context"
	"dash/data/repo"
)

type DeleteUserTheme struct {
	Repo repo.ThemeRepo
}

func NewDeleteUserTheme(r repo.ThemeRepo) *DeleteUserTheme {
	return &DeleteUserTheme{
		Repo: r,
	}
}

func (uc *DeleteUserTheme) Execute(ctx context.Context, userID string, id uint) error {
	list, err := uc.Repo.ListByUser(ctx, userID)
	if err != nil {
		return Internal("delete user theme: list", err)
	}
	if len(list) <= 1 {
		return nil
	}

	t, err := uc.Repo.GetByID(ctx, userID, id)
	if err != nil {
		return Internal("delete user theme: get by id", err)
	}
	if t == nil || !t.Deletable {
		return nil
	}
	if err := uc.Repo.Delete(ctx, userID, id); err != nil {
		return Internal("delete user theme: delete", err)
	}
	return nil
}
