package handler

import (
	"net/url"

	"github.com/gofiber/fiber/v2"
	"github.com/oechsler-it/dash/infra/oidc"
)

const (
	SessionLoginRoute          = "SessionLoginRoute"
	SessionLoginCallbackRoute  = "SessionLoginCallbackRoute"
	SessionLogoutRoute         = "SessionLogoutRoute"
	SessionLogoutCallbackRoute = "SessionLogoutCallbackRoute"
)

func Session(
	app *fiber.App,
	provider *oidc.Provider,
	store *oidc.SessionStore,
) {
	router := app.Group("/session")

	router.Get("/login", func(c *fiber.Ctx) error {
		state, err := oidc.GenerateState()
		if err != nil {
			return err
		}
		codeVerifier, err := oidc.GenerateCodeVerifier()
		if err != nil {
			return err
		}

		if err := store.SaveStateCookie(c, oidc.StateCookie{
			State:        state,
			CodeVerifier: codeVerifier,
			ReturnTo:     c.Query("rd", "/"),
		}); err != nil {
			return err
		}

		return c.Redirect(provider.BeginAuth(state, codeVerifier), fiber.StatusFound)
	}).Name(SessionLoginRoute)

	router.Get("/login/callback", func(c *fiber.Ctx) error {
		stateCookie, err := store.LoadAndClearStateCookie(c)
		if err != nil {
			return redirectToLogin(c)
		}

		if c.Query("state") != stateCookie.State {
			return redirectToLogin(c)
		}

		code := c.Query("code")
		if code == "" {
			return fiber.NewError(fiber.StatusBadRequest, "missing code")
		}

		idToken, rawIDToken, err := provider.Exchange(c.Context(), code, stateCookie.CodeVerifier)
		if err != nil {
			return fiber.NewError(fiber.StatusUnauthorized, "token exchange failed")
		}

		identity, err := provider.ClaimsToIdentity(idToken)
		if err != nil {
			return fiber.NewError(fiber.StatusUnauthorized, "failed to extract identity")
		}

		if err := store.Save(c, identity, rawIDToken, idToken.Expiry.Unix()); err != nil {
			return err
		}

		returnTo := stateCookie.ReturnTo
		if returnTo == "" {
			returnTo = "/"
		}
		return c.Redirect(returnTo, fiber.StatusFound)
	}).Name(SessionLoginCallbackRoute)

	router.Get("/logout", func(c *fiber.Ctx) error {
		sessionData, ok := store.Load(c)
		store.Clear(c)

		if !ok || sessionData.RawIDToken == "" {
			return redirectToLogin(c)
		}

		logoutCallbackURL, err := c.GetRouteURL(SessionLogoutCallbackRoute, fiber.Map{})
		if err != nil {
			return err
		}

		endSessionURL := provider.EndSessionURL(
			sessionData.RawIDToken,
			c.BaseURL()+logoutCallbackURL,
		)
		if endSessionURL == "" {
			return redirectToLogin(c)
		}

		return c.Redirect(endSessionURL, fiber.StatusFound)
	}).Name(SessionLogoutRoute)

	router.Get("/logout/callback", func(c *fiber.Ctx) error {
		loginURL, err := c.GetRouteURL(SessionLoginRoute, fiber.Map{})
		if err != nil {
			return err
		}
		return c.Redirect(loginURL, fiber.StatusFound)
	}).Name(SessionLogoutCallbackRoute)
}

func redirectToLogin(c *fiber.Ctx) error {
	loginURL, err := c.GetRouteURL(SessionLoginRoute, fiber.Map{})
	if err != nil {
		return err
	}

	u, err := url.Parse(loginURL)
	if err != nil {
		return c.Redirect(loginURL, fiber.StatusFound)
	}

	q := u.Query()
	q.Set("rd", c.Path())
	u.RawQuery = q.Encode()

	return c.Redirect(u.String(), fiber.StatusFound)
}
