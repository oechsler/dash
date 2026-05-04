package model

import "github.com/samber/lo"

// Identity represents the authenticated user making a request.
// UserID is always the OIDC sub claim — stable across username/email changes.
// It is a value object populated from the server-side session record.
type Identity struct {
	UserID      string   `json:"user_id"`
	FirstName   string   `json:"first_name"`
	LastName    string   `json:"last_name"`
	DisplayName string   `json:"display_name"`
	Username    string   `json:"username"`
	Email       string   `json:"email"`
	Picture     *string  `json:"picture"`
	Groups      []string `json:"groups"`
	IsAdmin     bool     `json:"is_admin"`
	ProfileUrl  *string  `json:"profile_url"`
}

// WithSyntheticGroups returns a copy of the identity with synthetic groups
// injected: dash_user for every authenticated user, dash_admin for admins.
// These groups allow applications to be scoped to all users or all admins
// without depending on IdP-specific group names.
func (i Identity) WithSyntheticGroups() Identity {
	groups := append([]string{"dash_user"}, i.Groups...)
	if i.IsAdmin {
		groups = append(groups, "dash_admin")
	}
	i.Groups = lo.Uniq(groups)
	return i
}
