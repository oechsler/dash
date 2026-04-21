package middleware

import (
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"

	"github.com/gofiber/fiber/v3"
)

// IdentityLoader is the HTTP-layer port for extracting an authenticated identity from a request.
// oidc.SessionStore implements this interface implicitly via its LoadIdentity method.
type IdentityLoader interface {
	LoadIdentity(c fiber.Ctx) (domainmodel.Identity, bool)
}

// LoadUserFromSession returns a Fiber middleware that loads the current user's identity
// from an encrypted session cookie. If no valid session is present, the request proceeds
// without a user set in context — handlers must call GetCurrentUser and handle the false case.
func LoadUserFromSession(loader IdentityLoader) fiber.Handler {
	return func(c fiber.Ctx) error {
		identity, ok := loader.LoadIdentity(c)
		if ok {
			c.Locals("user", identity)
		}
		return c.Next()
	}
}

// GetCurrentUser retrieves the authenticated identity from the request context.
// Returns false if no valid session was found for this request.
func GetCurrentUser(c fiber.Ctx) (domainmodel.Identity, bool) {
	identity, ok := c.Locals("user").(domainmodel.Identity)
	return identity, ok
}

// GetCurrentSessionPinned reports whether the current session is pinned.
// Set by LoadUserFromSession via SessionStore.LoadIdentity.
func GetCurrentSessionPinned(c fiber.Ctx) bool {
	pinned, _ := c.Locals("session_pinned").(bool)
	return pinned
}
