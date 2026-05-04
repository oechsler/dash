package query_test

import (
	"context"
	"errors"
	"testing"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/query"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

// ── ListUserThemes ─────────────────────────────────────────────────────────

func TestListUserThemes_Handle_RepoError(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return(nil, errors.New("db error"))

	h := query.NewListUserThemes(themeRepo)
	_, err := h.Handle(context.Background(), "user-1", uint(0))

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}

func TestListUserThemes_Handle_AlwaysIncludesDefault(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{}, nil)

	h := query.NewListUserThemes(themeRepo)
	themes, err := h.Handle(context.Background(), "user-1", uint(0))

	require.NoError(t, err)
	require.Len(t, themes, 1)
	require.Equal(t, uint(0), themes[0].ID)
	require.False(t, themes[0].Deletable)
}

func TestListUserThemes_Handle_FiltersSyntheticDuplicates(t *testing.T) {
	d := domainmodel.DefaultTheme()
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{
		// exact color duplicate — should be filtered
		{ID: 1, UserID: "user-1", DisplayName: "Old Default", Primary: d.Primary, Secondary: d.Secondary, Tertiary: d.Tertiary},
		// real user theme — should be kept
		{ID: 2, UserID: "user-1", DisplayName: "My Theme", Primary: "#aaaaaa", Secondary: "#bbbbbb", Tertiary: "#cccccc"},
	}, nil)

	h := query.NewListUserThemes(themeRepo)
	themes, err := h.Handle(context.Background(), "user-1", uint(0))

	require.NoError(t, err)
	require.Len(t, themes, 2) // default + My Theme
	require.Equal(t, uint(0), themes[0].ID)
	require.Equal(t, "My Theme", themes[1].Name)
}

func TestListUserThemes_Handle_UserThemeDeletable(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{
		{ID: 3, DisplayName: "Custom", Primary: "#111", Secondary: "#222", Tertiary: "#333"},
	}, nil)

	h := query.NewListUserThemes(themeRepo)
	themes, err := h.Handle(context.Background(), "user-1", uint(0))

	require.NoError(t, err)
	require.Len(t, themes, 2)
	require.True(t, themes[1].Deletable)
}

func TestListUserThemes_Handle_ActiveThemeNotDeletable(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("ListByUser", mock.Anything, "user-1").Return([]domainrepo.ThemeRecord{
		{ID: 3, DisplayName: "Active", Primary: "#111", Secondary: "#222", Tertiary: "#333"},
		{ID: 4, DisplayName: "Other", Primary: "#aaa", Secondary: "#bbb", Tertiary: "#ccc"},
	}, nil)

	h := query.NewListUserThemes(themeRepo)
	themes, err := h.Handle(context.Background(), "user-1", uint(3)) // theme 3 is active

	require.NoError(t, err)
	require.Len(t, themes, 3) // default + Active + Other
	active := themes[1]
	other := themes[2]
	require.Equal(t, uint(3), active.ID)
	require.False(t, active.Deletable)
	require.True(t, other.Deletable)
}

// ── GetUserThemeByID ───────────────────────────────────────────────────────

func TestGetUserThemeByID_Handle_DefaultThemeID(t *testing.T) {
	h := query.NewGetUserThemeByID(nil)
	theme, err := h.Handle(context.Background(), "user-1", 0)

	require.NoError(t, err)
	require.Equal(t, uint(0), theme.ID)
	require.False(t, theme.Deletable)
}

func TestGetUserThemeByID_Handle_RegularTheme(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(3)).Return(&domainrepo.ThemeRecord{
		ID:          3,
		UserID:      "user-1",
		DisplayName: "My Theme",
		Primary:     "#111111",
		Secondary:   "#222222",
		Tertiary:    "#333333",
	}, nil)

	h := query.NewGetUserThemeByID(themeRepo)
	theme, err := h.Handle(context.Background(), "user-1", 3)

	require.NoError(t, err)
	require.Equal(t, uint(3), theme.ID)
	require.Equal(t, "My Theme", theme.Name)
	require.True(t, theme.Deletable)
}

func TestGetUserThemeByID_Handle_SyntheticDuplicate_ReturnsDefault(t *testing.T) {
	d := domainmodel.DefaultTheme()
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(3)).Return(&domainrepo.ThemeRecord{
		ID:          3,
		DisplayName: d.Name,
		Primary:     d.Primary,
		Secondary:   d.Secondary,
		Tertiary:    d.Tertiary,
	}, nil)

	h := query.NewGetUserThemeByID(themeRepo)
	theme, err := h.Handle(context.Background(), "user-1", 3)

	require.NoError(t, err)
	require.Equal(t, uint(0), theme.ID) // normalised to synthetic default
}

func TestGetUserThemeByID_Handle_NotFound(t *testing.T) {
	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("GetByID", mock.Anything, "user-1", uint(99)).
		Return(nil, domainerrors.NotFound(domainerrors.EntityTheme))

	h := query.NewGetUserThemeByID(themeRepo)
	_, err := h.Handle(context.Background(), "user-1", 99)

	var nfe *domainerrors.NotFoundError
	require.ErrorAs(t, err, &nfe)
}
