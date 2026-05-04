package model

import "testing"

func TestBookmarkUpdateIcon(t *testing.T) {
	icon, _ := NewIcon("mdi", "home")
	b := Bookmark{DisplayName: "Test", CategoryID: 1}
	b.UpdateIcon(icon)
	if b.Icon != icon {
		t.Errorf("UpdateIcon: got %v, want %v", b.Icon, icon)
	}
}

func TestBookmarkRename(t *testing.T) {
	b := Bookmark{DisplayName: "Old"}
	b.Rename("New Name")
	if b.DisplayName != "New Name" {
		t.Errorf("Rename: got %q, want %q", b.DisplayName, "New Name")
	}
}

func TestBookmarkChangeURL(t *testing.T) {
	newURL, _ := ParseBookmarkURL("https://new.example.com")
	b := Bookmark{DisplayName: "Test"}
	b.ChangeURL(newURL)
	if b.Url != newURL {
		t.Errorf("ChangeURL: got %v, want %v", b.Url, newURL)
	}
}

func TestBookmarkMoveTo(t *testing.T) {
	b := Bookmark{CategoryID: 1}
	b.MoveTo(42)
	if b.CategoryID != 42 {
		t.Errorf("MoveTo: got %d, want 42", b.CategoryID)
	}
}
