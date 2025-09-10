package middleware

import (
	"dash/environment"
	"strings"
	"time"

	"github.com/gofiber/fiber/v2"
	"github.com/golang-jwt/jwt/v5"
	"github.com/samber/lo"
)

type User struct {
	ID          string   `json:"id"`
	FirstName   string   `json:"firstName"`
	LastName    string   `json:"lastName"`
	DisplayName string   `json:"displayName"`
	Username    string   `json:"username"`
	Email       string   `json:"email"`
	Picture     *string  `json:"picture"`
	Groups      []string `json:"groups"`
	IsAdmin     bool     `json:"isAdmin"`
	ProfileUrl  *string  `json:"profileUrl"`
}

func GetUserFromIdToken(env *environment.Env) fiber.Handler {
	return func(c *fiber.Ctx) error {
		authorizationHeader := c.Get("Authorization")
		if authorizationHeader == "" {
			// No Authorization header: try to gather data from forward-auth headers set by the proxy.
			// We try to collect as much as possible and apply fallbacks:
			// - user id: prefer username, then user, then email
			// - display_name -> username if needed
			// - first name -> username if needed
			hUser := c.Get("X-Auth-Request-User")
			hGroups := c.Get("X-Auth-Request-Groups")
			hEmail := c.Get("X-Auth-Request-Email")
			hUsername := c.Get("X-Auth-Request-Preferred-Username")

			// Parse groups (comma-separated list).
			groups := make([]string, 0)
			if hGroups != "" {
				for _, g := range strings.Split(hGroups, ",") {
					g = strings.TrimSpace(g)
					if g != "" {
						groups = append(groups, g)
					}
				}
			}

			// User ID: prefer username, then user, then email
			id := ""
			switch {
			case hUsername != "":
				id = hUsername
			case hUser != "":
				id = hUser
			case hEmail != "":
				id = hEmail
			}

			// Names with fallbacks
			firstName := ""
			lastName := ""
			username := hUsername
			if username == "" {
				username = hUser
			}
			displayName := ""
			if username != "" {
				// If nothing better is available: displayName -> username, firstName -> username
				displayName = username
				firstName = username
			}

			adminGroup := env.String("OAUTH2_ADMIN_GROUP", "admin")
			isAdmin := adminGroup == "*" || lo.Contains(groups, adminGroup)

			profileUrl := env.String("OAUTH2_PROFILE_URL", "")

			c.Locals("user", User{
				ID:          id,
				FirstName:   firstName,
				LastName:    lastName,
				DisplayName: displayName,
				Username:    username,
				Email:       hEmail,
				Picture:     nil, // No picture header available here
				Groups:      groups,
				IsAdmin:     isAdmin,
				ProfileUrl: func() *string {
					if profileUrl != "" {
						return &profileUrl
					}
					return nil
				}(),
			})

			return c.Next()
		}

		// Authorization header present: parse ID token and extract claims.
		idToken, _ := strings.CutPrefix(authorizationHeader, "Bearer ")
		c.Locals("id_token", idToken)

		token, _ := jwt.Parse(idToken, func(token *jwt.Token) (interface{}, error) {
			// No verification here; we only read claims to build a user context.
			return nil, nil
		})
		claims := token.Claims.(jwt.MapClaims)

		// Check if the token has expired
		// This is the only validation check we do,
		// proxy should do all other checks
		if exp, ok := claims["exp"].(float64); ok {
			if float64(time.Now().Unix()) > exp {
				return c.Next()
			}
		} else {
			return c.Next()
		}

		// Try extracting user data from claims
		id, _ := claims["sub"].(string)
		firstName, _ := claims["given_name"].(string)
		lastName, _ := claims["family_name"].(string)
		username, _ := claims["preferred_username"].(string)
		email, _ := claims["email"].(string)
		picture, _ := claims["picture"].(string)

		// Groups may arrive as []any
		rawGroups, ok := claims["groups"].([]interface{})
		groups := make([]string, 0)
		if ok {
			groups = make([]string, len(rawGroups))
			for i, v := range rawGroups {
				if str, ok := v.(string); ok {
					groups[i] = str
				}
			}
		}

		// ID fallback: if sub is empty, use username, then user, then email
		if id == "" {
			if username != "" {
				id = username
			} else if email != "" {
				id = email
			}
		}

		// FirstName fallback: if empty, use username.
		if firstName == "" && username != "" {
			firstName = username
		}

		// DisplayName: prefer "FirstName LastName" (when available), otherwise username.
		displayName := ""
		if firstName != "" || lastName != "" {
			name := firstName
			if name == "" {
				name = username
			}
			if lastName != "" {
				displayName = name + " " + lastName
			} else {
				displayName = name
			}
		} else {
			displayName = username
		}

		adminGroup := env.String("OAUTH2_ADMIN_GROUP", "admin")
		isAdmin := adminGroup == "*" || lo.Contains(groups, adminGroup)

		profileUrl := env.String("OAUTH2_PROFILE_URL", "")

		c.Locals("user", User{
			ID:          id,
			FirstName:   firstName,
			LastName:    lastName,
			DisplayName: displayName,
			Username:    username,
			Email:       email,
			Picture: func() *string {
				if picture != "" {
					return &picture
				}
				return nil
			}(),
			Groups:  groups,
			IsAdmin: isAdmin,
			ProfileUrl: func() *string {
				if profileUrl != "" {
					return &profileUrl
				}
				return nil
			}(),
		})

		return c.Next()
	}
}

func GetCurrentUser(c *fiber.Ctx) (User, bool) {
	user, ok := c.Locals("user").(User)
	return user, ok
}
