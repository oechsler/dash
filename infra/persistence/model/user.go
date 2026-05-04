package model

// User is the relational anchor for all user-owned data.
// It carries only the internal user ID — all profile data lives in the
// session record (loaded from the IdP on login) or in user_idp_links.
type User struct {
	ID string `gorm:"primaryKey"`
}

func (u *User) TableName() string { return "users" }
