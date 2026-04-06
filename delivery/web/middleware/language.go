package middleware

import (
	"strings"

	"github.com/invopop/ctxi18n"
	"git.at.oechsler.it/samuel/dash/v2/app/query"

	"github.com/gofiber/fiber/v2"
)

// WithLanguage resolves the effective locale for each request and injects it into
// the user context so Templ templates can call i18n.T(ctx, "key") directly.
//
// Resolution order:
//  1. User's stored language preference (from settings DB) — if not "auto" or empty
//  2. Browser's Accept-Language header
//  3. Fall back to English
func WithLanguage(loader IdentityLoader, getUserSettings query.UserSettingsGetter) fiber.Handler {
	return func(c *fiber.Ctx) error {
		lang := resolveLanguage(c, loader, getUserSettings)
		ctx, err := ctxi18n.WithLocale(c.UserContext(), lang)
		if err != nil {
			ctx, _ = ctxi18n.WithLocale(c.UserContext(), "en")
		}
		c.SetUserContext(ctx)
		return c.Next()
	}
}

func resolveLanguage(c *fiber.Ctx, loader IdentityLoader, getUserSettings query.UserSettingsGetter) string {
	// Try to get the user's stored language preference.
	if identity, ok := loader.LoadIdentity(c); ok && getUserSettings != nil {
		if settings, err := getUserSettings.Handle(c.UserContext(), identity.UserID); err == nil {
			stored := settings.Language
			if stored != "" && stored != "auto" {
				return stored
			}
		}
	}

	// Fall back to Accept-Language header.
	return parseAcceptLanguage(c.Get("Accept-Language"))
}

// parseAcceptLanguage returns "de" if the browser prefers any German variant,
// otherwise returns "en".
func parseAcceptLanguage(header string) string {
	for _, part := range strings.Split(header, ",") {
		tag := strings.TrimSpace(strings.SplitN(part, ";", 2)[0])
		tag = strings.ToLower(tag)
		if tag == "de" || strings.HasPrefix(tag, "de-") {
			return "de"
		}
	}
	return "en"
}
