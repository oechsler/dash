package model

import "testing"

func TestNewUserDashboard(t *testing.T) {
	ud := NewUserDashboard(42, "user-abc")

	if ud.ID() != 42 {
		t.Errorf("ID() = %d, want 42", ud.ID())
	}
	if ud.UserID() != "user-abc" {
		t.Errorf("UserID() = %q, want \"user-abc\"", ud.UserID())
	}
}

func TestUserDashboard_OwnedBy(t *testing.T) {
	ud := NewUserDashboard(1, "user-abc")

	if !ud.OwnedBy("user-abc") {
		t.Error("OwnedBy should return true for the owning user")
	}
	if ud.OwnedBy("other-user") {
		t.Error("OwnedBy should return false for a different user")
	}
	if ud.OwnedBy("") {
		t.Error("OwnedBy should return false for empty string")
	}
}

func TestUserDashboard_OwnsCategory(t *testing.T) {
	ud := NewUserDashboard(5, "user-abc")

	if !ud.OwnsCategory(5) {
		t.Error("OwnsCategory should return true when dashboardID matches")
	}
	if ud.OwnsCategory(6) {
		t.Error("OwnsCategory should return false when dashboardID differs")
	}
	if ud.OwnsCategory(0) {
		t.Error("OwnsCategory should return false for zero ID")
	}
}
