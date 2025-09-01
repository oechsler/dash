package middleware

import (
	"strings"

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
	Picture     string   `json:"picture"`
	Groups      []string `json:"groups"`
	IsAdmin     bool     `json:"isAdmin"`
}

func GetUserFromIdToken(c *fiber.Ctx) error {
	authorizationHeader := c.Get("Authorization")
	if authorizationHeader == "" {
		return c.Next()
	}
	idToken, _ := strings.CutPrefix(authorizationHeader, "Bearer ")
	c.Locals("id_token", idToken)

	token, _ := jwt.Parse(idToken, func(token *jwt.Token) (interface{}, error) {
		return nil, nil
	})
	claims := token.Claims.(jwt.MapClaims)

	id := claims["sub"].(string)
	firstName, _ := claims["given_name"].(string)
	lastName, _ := claims["family_name"].(string)
	username, _ := claims["preferred_username"].(string)
	email, _ := claims["email"].(string)
	picture, _ := claims["picture"].(string)
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

	isAdmin := lo.Contains(groups, "admin")

	c.Locals("user", User{
		ID:        id,
		FirstName: firstName,
		LastName:  lastName,
		DisplayName: firstName + func() string {
			if lastName != "" {
				return " " + lastName
			}
			return ""
		}(),
		Username: username,
		Email:    email,
		Picture:  picture,
		Groups:   groups,
		IsAdmin:  isAdmin,
	})

	return c.Next()
}

func GetCurrentUser(c *fiber.Ctx) (User, bool) {
	user, ok := c.Locals("user").(User)
	return user, ok
}
