package query

import (
	"context"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
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
	if domainmodel.IsDefaultThemeID(id) {
		t := domainmodel.DefaultTheme()
		return &t, nil
	}
	t, err := h.Repo.GetByID(ctx, userID, id)
	if err != nil {
		return nil, domainerrors.WrapRepo("get theme by id", err)
	}
	// If the stored theme is identical to the synthetic default, return the
	// synthetic so callers always work with ID=0 for the default.
	if domainmodel.IsSyntheticDuplicate(t.DisplayName, t.Primary, t.Secondary, t.Tertiary) {
		def := domainmodel.DefaultTheme()
		return &def, nil
	}
	return &domainmodel.Theme{
		ID:        t.ID,
		Name:      t.DisplayName,
		Primary:   t.Primary,
		Secondary: t.Secondary,
		Tertiary:  t.Tertiary,
		Deletable: true,
	}, nil
}
