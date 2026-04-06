package model

type Theme struct {
	ID        uint   `json:"id"`
	Name      string `json:"name"`
	Primary   string `json:"primary"`
	Secondary string `json:"secondary"`
	Tertiary  string `json:"tertiary"`
	Deletable bool   `json:"deletable"`
}

// CanDelete reports whether this theme may be deleted by the user.
func (t Theme) CanDelete() bool { return t.Deletable }

// IsSystem reports whether this is a non-deletable system theme.
func (t Theme) IsSystem() bool { return !t.Deletable }
