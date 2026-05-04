package command_test

import (
	"context"
	"errors"
	"testing"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	repoMock "git.at.oechsler.it/samuel/dash/v2/internal/mock"
)

func validThemeCmd() command.CreateUserThemeCmd {
	return command.CreateUserThemeCmd{
		DisplayName: "My Theme",
		Primary:     "#1e1e2e",
		Secondary:   "#cdd6f4",
		Tertiary:    "#cba6f7",
	}
}

func TestCreateUserTheme_Handle_ValidationError(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(errors.New("failed"))

	h := command.NewCreateUserTheme(nil, v)
	err := h.Handle(context.Background(), "user-1", command.CreateUserThemeCmd{})

	var ve *domainerrors.ValidationError
	require.ErrorAs(t, err, &ve)
}

func TestCreateUserTheme_Handle_Success(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("Create", mock.Anything, mock.MatchedBy(func(r interface{}) bool { return r != nil })).
		Return(nil)

	h := command.NewCreateUserTheme(themeRepo, v)
	err := h.Handle(context.Background(), "user-1", validThemeCmd())

	require.NoError(t, err)
	themeRepo.AssertExpectations(t)
}

func TestCreateUserTheme_Handle_RepoError(t *testing.T) {
	v := &repoMock.Validator{}
	v.On("Struct", mock.Anything).Return(nil)

	themeRepo := &repoMock.ThemeRepository{}
	themeRepo.On("Create", mock.Anything, mock.Anything).Return(errors.New("db error"))

	h := command.NewCreateUserTheme(themeRepo, v)
	err := h.Handle(context.Background(), "user-1", validThemeCmd())

	var ie *domainerrors.InternalError
	require.ErrorAs(t, err, &ie)
}
