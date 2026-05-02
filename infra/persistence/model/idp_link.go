package model

import "time"

// IdpLink maps an OIDC (issuer, sub) identity to an internal UserID.
// A user can have multiple links (one per connected IdP) all pointing to
// the same UserID, giving them a single unified account.
type IdpLink struct {
	UserID    string    `gorm:"not null;index"`
	Issuer    string    `gorm:"not null;primaryKey"`
	Sub       string    `gorm:"not null;primaryKey"`
	IsPrimary bool      `gorm:"not null;default:true"`
	LinkedAt  time.Time `gorm:"not null"`
	User      User      `gorm:"constraint:fk_idp_links_user,OnDelete:CASCADE"`
}

func (IdpLink) TableName() string { return "user_idp_links" }
