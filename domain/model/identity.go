package model

// Identity represents the authenticated user making a request.
// It is a value object populated from the OIDC session cookie — no database record exists for it.
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
