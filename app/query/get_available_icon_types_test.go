package query_test

import (
	"context"
	"testing"

	"github.com/stretchr/testify/require"

	"git.at.oechsler.it/samuel/dash/v2/app/query"
)

func TestGetAvailableIconTypes_Handle(t *testing.T) {
	h := query.NewGetAvailableIconTypes()
	types, err := h.Handle(context.Background())

	require.NoError(t, err)
	require.NotEmpty(t, types)
	require.Contains(t, types, "mdi")
	require.Contains(t, types, "spi")
}
