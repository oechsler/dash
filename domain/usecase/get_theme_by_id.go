package usecase

import (
	"context"
	"dash/data/repo"
	"dash/domain/model"
)

type GetUserThemeByID struct{ Repo repo.ThemeRepo }

func NewGetUserThemeByID(r repo.ThemeRepo) *GetUserThemeByID { return &GetUserThemeByID{Repo: r} }
func (uc *GetUserThemeByID) Execute(ctx context.Context, userID string, id uint) (*model.Theme, error) {
	t, err := uc.Repo.GetByID(ctx, userID, id)
	if err != nil {
		return nil, err
	}
	return &model.Theme{ID: t.ID, Name: t.Name, Primary: t.Primary, Secondary: t.Secondary, Tertiary: t.Tertiary}, nil
}
