package model

import "testing"

func TestNewCategory(t *testing.T) {
	tests := []struct {
		input       string
		wantErr     bool
		wantDisplay string
	}{
		{"Work", false, "Work"},
		{"  trimmed  ", false, "trimmed"},
		{"", true, ""},
		{"   ", true, ""},
		{"\t\n", true, ""},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			c, err := NewCategory(tt.input)
			if tt.wantErr {
				if err == nil {
					t.Errorf("NewCategory(%q) expected error, got nil", tt.input)
				}
			} else {
				if err != nil {
					t.Errorf("NewCategory(%q) unexpected error: %v", tt.input, err)
				}
				if c.DisplayName != tt.wantDisplay {
					t.Errorf("DisplayName = %q, want %q", c.DisplayName, tt.wantDisplay)
				}
				if c.IsShelved {
					t.Error("new category should not be shelved")
				}
			}
		})
	}
}

func TestCategoryShelveUnshelve(t *testing.T) {
	c := Category{DisplayName: "Test"}

	c.Shelve()
	if !c.IsShelved {
		t.Error("Shelve() should set IsShelved = true")
	}

	c.Unshelve()
	if c.IsShelved {
		t.Error("Unshelve() should set IsShelved = false")
	}
}

func TestCategoryRename(t *testing.T) {
	tests := []struct {
		input       string
		wantErr     bool
		wantDisplay string
	}{
		{"New Name", false, "New Name"},
		{"  padded  ", false, "padded"},
		{"", true, ""},
		{"   ", true, ""},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			c := Category{DisplayName: "Original"}
			err := c.Rename(tt.input)
			if tt.wantErr {
				if err == nil {
					t.Errorf("Rename(%q) expected error, got nil", tt.input)
				}
				if c.DisplayName != "Original" {
					t.Errorf("Rename on error should not mutate DisplayName, got %q", c.DisplayName)
				}
			} else {
				if err != nil {
					t.Errorf("Rename(%q) unexpected error: %v", tt.input, err)
				}
				if c.DisplayName != tt.wantDisplay {
					t.Errorf("DisplayName = %q, want %q", c.DisplayName, tt.wantDisplay)
				}
			}
		})
	}
}
