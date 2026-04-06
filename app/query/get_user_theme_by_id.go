package query

import (
	"context"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainmodel "github.com/oechsler-it/dash/domain/model"
	domainrepo "github.com/oechsler-it/dash/domain/repo"
)

// UserThemeByIDGetter handles the get-user-theme-by-id query.
type UserThemeByIDGetter interface {
	Handle(ctx context.Context, userID string, id uint) (*domainmodel.Theme, error)
}

type GetUserThemeByID struct {
	Repo domainrepo.ThemeRepository
}

func NewGetUserThemeByID(r domainrepo.ThemeRepository) *GetUserThemeByID {
	return &GetUserThemeByID{Repo: r}
}

func (h *GetUserThemeByID) Handle(ctx context.Context, userID string, id uint) (*domainmodel.Theme, error) {
	t, err := h.Repo.GetByID(ctx, userID, id)
	if err != nil {
		return nil, domainerrors.WrapRepo("get theme by id", err)
	}
	return &domainmodel.Theme{
		ID:        t.ID,
		Name:      t.DisplayName,
		Primary:   t.Primary,
		Secondary: t.Secondary,
		Tertiary:  t.Tertiary,
	}, nil
}
