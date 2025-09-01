package usecase

import (
    "context"
    datamodel "dash/data/model"
    "dash/data/repo"
    dom "dash/domain/model"
)

type ListUserThemes struct { Repo repo.ThemeRepo }
func NewListUserThemes(r repo.ThemeRepo) *ListUserThemes { return &ListUserThemes{Repo: r} }
func (uc *ListUserThemes) Execute(ctx context.Context, userID string) ([]dom.Theme, error) {
    list, err := uc.Repo.ListByUser(ctx, userID)
    if err != nil { return nil, err }
    out := make([]dom.Theme, 0, len(list))
    for _, t := range list {
        out = append(out, dom.Theme{ID: t.ID, Name: t.Name, Primary: t.Primary, Secondary: t.Secondary, Tertiary: t.Tertiary, Deletable: t.Deletable})
    }
    return out, nil
}

type CreateUserTheme struct { Repo repo.ThemeRepo }
func NewCreateUserTheme(r repo.ThemeRepo) *CreateUserTheme { return &CreateUserTheme{Repo: r} }
func (uc *CreateUserTheme) Execute(ctx context.Context, userID, name, primary, secondary, tertiary string) (*dom.Theme, error) {
    t := &datamodel.Theme{UserId: userID, Name: name, Primary: primary, Secondary: secondary, Tertiary: tertiary, Deletable: true}
    if err := uc.Repo.Create(ctx, t); err != nil { return nil, err }
    return &dom.Theme{ID: t.ID, Name: t.Name, Primary: t.Primary, Secondary: t.Secondary, Tertiary: t.Tertiary, Deletable: t.Deletable}, nil
}

type DeleteUserTheme struct { Repo repo.ThemeRepo }
func NewDeleteUserTheme(r repo.ThemeRepo) *DeleteUserTheme { return &DeleteUserTheme{Repo: r} }
func (uc *DeleteUserTheme) Execute(ctx context.Context, userID string, id uint) error {
    // ensure at least one remains after delete
    list, err := uc.Repo.ListByUser(ctx, userID)
    if err != nil { return err }
    if len(list) <= 1 { return nil }
    // ensure theme is deletable
    t, err := uc.Repo.GetByID(ctx, userID, id)
    if err != nil { return err }
    if t == nil || !t.Deletable { return nil }
    return uc.Repo.Delete(ctx, userID, id)
}

type EnsureDefaultTheme struct { Repo repo.ThemeRepo }
func NewEnsureDefaultTheme(r repo.ThemeRepo) *EnsureDefaultTheme { return &EnsureDefaultTheme{Repo: r} }
func (uc *EnsureDefaultTheme) Execute(ctx context.Context, userID string) (*dom.Theme, error) {
    t, err := uc.Repo.EnsureDefault(ctx, userID)
    if err != nil { return nil, err }
    return &dom.Theme{ID: t.ID, Name: t.Name, Primary: t.Primary, Secondary: t.Secondary, Tertiary: t.Tertiary, Deletable: t.Deletable}, nil
}
