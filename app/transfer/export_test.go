package transfer

import (
	"testing"
	"time"

	"github.com/stretchr/testify/require"
)

func sampleExport() *UserDataExport {
	return &UserDataExport{
		Version:    1,
		ExportedAt: time.Date(2026, 1, 1, 0, 0, 0, 0, time.UTC),
		Username:   "sam",
		Settings: SettingsExport{
			Language:  "de",
			Timezone:  "Europe/Vienna",
			ThemeName: "My Theme",
		},
		Themes: []ThemeExport{
			{Name: "My Theme", Primary: "#111", Secondary: "#222", Tertiary: "#333"},
		},
		Categories: []CategoryExport{
			{
				DisplayName: "Work",
				Bookmarks: []BookmarkExport{
					{Icon: "mdi:link", DisplayName: "GitHub", URL: "https://github.com"},
				},
			},
		},
	}
}

func TestContentHash_Deterministic(t *testing.T) {
	h1 := ContentHash("a", "b", "c")
	h2 := ContentHash("a", "b", "c")
	require.Equal(t, h1, h2)
}

func TestContentHash_DifferentInputs(t *testing.T) {
	h1 := ContentHash("a", "b")
	h2 := ContentHash("a", "c")
	require.NotEqual(t, h1, h2)
}

func TestContentHash_Length(t *testing.T) {
	h := ContentHash("test")
	require.Len(t, h, 16)
}

func TestMarshalExport_RoundTrip(t *testing.T) {
	export := sampleExport()
	data, err := MarshalExport(export)
	require.NoError(t, err)
	require.NotEmpty(t, data)

	parsed, err := UnmarshalExport(data)
	require.NoError(t, err)
	require.Equal(t, export.Version, parsed.Version)
	require.Equal(t, export.Username, parsed.Username)
	require.Equal(t, export.Settings.Language, parsed.Settings.Language)
	require.Len(t, parsed.Themes, 1)
	require.Len(t, parsed.Categories, 1)
}

func TestMarshalExport_SetsSignature(t *testing.T) {
	export := sampleExport()
	_, err := MarshalExport(export)
	require.NoError(t, err)
	require.NotEmpty(t, export.Signature)
}

func TestUnmarshalExport_MissingSignature(t *testing.T) {
	data := []byte(`{"version":1,"exported_at":"2026-01-01T00:00:00Z","username":"sam","settings":{"language":"","timezone":""},"themes":[],"categories":[]}`)
	_, err := UnmarshalExport(data)
	require.ErrorIs(t, err, ErrMissingSignature)
}

func TestUnmarshalExport_InvalidSignature(t *testing.T) {
	export := sampleExport()
	data, err := MarshalExport(export)
	require.NoError(t, err)

	// Tamper: replace a character in the JSON
	tampered := append([]byte{}, data...)
	tampered[50] ^= 0x01
	_, err = UnmarshalExport(tampered)
	require.Error(t, err) // either JSON error or invalid signature
}

func TestUnmarshalExport_WrongSignature(t *testing.T) {
	export := sampleExport()
	data, err := MarshalExport(export)
	require.NoError(t, err)

	// Replace the signature value in the JSON with a wrong one
	// Find and replace the signature with a known bad value
	bad := make([]byte, len(data))
	copy(bad, data)
	// Parse and re-sign with wrong value by manipulating the struct
	parsed, err := UnmarshalExport(data)
	require.NoError(t, err)
	parsed.Signature = "aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011"
	encoded, err := MarshalExport(parsed) // this sets a new valid sig
	require.NoError(t, err)

	// Manually set wrong signature in JSON bytes (simulate corruption)
	_ = encoded
	// Direct test: create JSON with wrong signature
	wrongSig := `{"version":1,"exported_at":"2026-01-01T00:00:00Z","username":"sam","settings":{"language":"","timezone":""},"themes":[],"categories":[],"signature":"0000000000000000000000000000000000000000000000000000000000000000"}`
	_, err = UnmarshalExport([]byte(wrongSig))
	require.ErrorIs(t, err, ErrInvalidSignature)
}

func TestUnmarshalExport_InvalidJSON(t *testing.T) {
	_, err := UnmarshalExport([]byte("not json"))
	require.Error(t, err)
}
