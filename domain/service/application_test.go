package service

import (
	"testing"

	"git.at.oechsler.it/samuel/dash/v2/domain/model"
)

func makeApp(id uint, groups ...string) model.AppLink {
	return model.AppLink{
		ID:              id,
		DisplayName:     "App",
		VisibleToGroups: groups,
	}
}

func TestFilterForUser_NoGroupRestriction(t *testing.T) {
	apps := []model.AppLink{makeApp(1), makeApp(2)}
	result := FilterForUser(apps, []string{"dev"})
	if len(result) != 2 {
		t.Errorf("apps with no groups should be visible to everyone, got %d", len(result))
	}
}

func TestFilterForUser_UserHasMembership(t *testing.T) {
	apps := []model.AppLink{makeApp(1, "dev", "ops")}
	result := FilterForUser(apps, []string{"dev"})
	if len(result) != 1 {
		t.Errorf("user in group should see the app, got %d", len(result))
	}
}

func TestFilterForUser_UserLacksMembership(t *testing.T) {
	apps := []model.AppLink{makeApp(1, "admin")}
	result := FilterForUser(apps, []string{"dev"})
	if len(result) != 0 {
		t.Errorf("user not in group should not see the app, got %d", len(result))
	}
}

func TestFilterForUser_EmptyApps(t *testing.T) {
	result := FilterForUser(nil, []string{"dev"})
	if len(result) != 0 {
		t.Errorf("empty app list should return empty result, got %d", len(result))
	}
}

func TestFilterForUser_EmptyUserGroups(t *testing.T) {
	apps := []model.AppLink{
		makeApp(1),              // no restriction — should be visible
		makeApp(2, "dev"),       // restricted — not visible
	}
	result := FilterForUser(apps, []string{})
	if len(result) != 1 {
		t.Errorf("user with no groups should only see unrestricted apps, got %d", len(result))
	}
	if result[0].ID != 1 {
		t.Errorf("wrong app visible: got ID %d, want 1", result[0].ID)
	}
}

func TestFilterForUser_Mixed(t *testing.T) {
	apps := []model.AppLink{
		makeApp(1),              // visible to all
		makeApp(2, "dev"),       // user is in dev
		makeApp(3, "admin"),     // user not in admin
		makeApp(4, "dev", "ops"), // user is in dev
	}
	result := FilterForUser(apps, []string{"dev"})
	if len(result) != 3 {
		t.Errorf("expected 3 visible apps, got %d", len(result))
	}
}

func TestFilterForUser_NilUserGroups(t *testing.T) {
	apps := []model.AppLink{
		makeApp(1),        // visible to all
		makeApp(2, "dev"), // restricted
	}
	result := FilterForUser(apps, nil)
	if len(result) != 1 {
		t.Errorf("nil user groups should only see unrestricted apps, got %d", len(result))
	}
}
