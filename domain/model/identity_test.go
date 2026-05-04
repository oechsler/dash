package model

import (
	"slices"
	"testing"
)

func TestWithSyntheticGroups_NonAdmin(t *testing.T) {
	id := Identity{
		UserID:  "user-1",
		Groups:  []string{"dev", "ops"},
		IsAdmin: false,
	}
	result := id.WithSyntheticGroups()

	if !slices.Contains(result.Groups, "dash_user") {
		t.Error("non-admin should have dash_user group")
	}
	if slices.Contains(result.Groups, "dash_admin") {
		t.Error("non-admin should not have dash_admin group")
	}
	if !slices.Contains(result.Groups, "dev") {
		t.Error("original groups should be preserved")
	}
	if !slices.Contains(result.Groups, "ops") {
		t.Error("original groups should be preserved")
	}
}

func TestWithSyntheticGroups_Admin(t *testing.T) {
	id := Identity{
		UserID:  "admin-1",
		Groups:  []string{"admins"},
		IsAdmin: true,
	}
	result := id.WithSyntheticGroups()

	if !slices.Contains(result.Groups, "dash_user") {
		t.Error("admin should also have dash_user group")
	}
	if !slices.Contains(result.Groups, "dash_admin") {
		t.Error("admin should have dash_admin group")
	}
	if !slices.Contains(result.Groups, "admins") {
		t.Error("original groups should be preserved")
	}
}

func TestWithSyntheticGroups_Deduplication(t *testing.T) {
	id := Identity{
		UserID:  "user-1",
		Groups:  []string{"dash_user", "dash_admin"},
		IsAdmin: true,
	}
	result := id.WithSyntheticGroups()

	count := 0
	for _, g := range result.Groups {
		if g == "dash_user" {
			count++
		}
	}
	if count != 1 {
		t.Errorf("dash_user appears %d times, want 1", count)
	}

	count = 0
	for _, g := range result.Groups {
		if g == "dash_admin" {
			count++
		}
	}
	if count != 1 {
		t.Errorf("dash_admin appears %d times, want 1", count)
	}
}

func TestWithSyntheticGroups_DoesNotMutateOriginal(t *testing.T) {
	original := []string{"dev"}
	id := Identity{Groups: original, IsAdmin: false}
	_ = id.WithSyntheticGroups()

	if len(id.Groups) != 1 || id.Groups[0] != "dev" {
		t.Error("WithSyntheticGroups must not mutate the original identity")
	}
}

func TestWithSyntheticGroups_EmptyGroups(t *testing.T) {
	id := Identity{UserID: "user-1", Groups: nil, IsAdmin: false}
	result := id.WithSyntheticGroups()

	if !slices.Contains(result.Groups, "dash_user") {
		t.Error("user with no groups should still get dash_user")
	}
}
