package handler

import (
	"net/url"
	"time"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	"git.at.oechsler.it/samuel/dash/v2/infra/oidc"

	"github.com/gofiber/fiber/v3"
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
	createSession command.SessionCreator,
) {
	router := app.Group("/session")

	router.Get("/login", func(c fiber.Ctx) error {
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

		return c.Redirect().Status(fiber.StatusFound).To(provider.BeginAuth(state, codeVerifier))
	}).Name(SessionLoginRoute)

	router.Get("/login/callback", func(c fiber.Ctx) error {
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

		sessionData, err := store.Save(c, identity, rawIDToken, idToken.Expiry.Unix())
		if err != nil {
			return err
		}

		// Persist the session to the DB so it appears in the session overview.
		// PinnedUntil is zero until the user explicitly pins it.
		// Ignore errors — a DB hiccup must not prevent login.
		if createSession != nil {
			pic := ""
			if identity.Picture != nil {
				pic = *identity.Picture
			}
			pu := ""
			if identity.ProfileUrl != nil {
				pu = *identity.ProfileUrl
			}
			_ = createSession.Handle(c.Context(), command.CreateSessionCmd{
				SessionID:   sessionData.SessionID,
				UserID:      identity.UserID,
				IssuedAt:    idToken.IssuedAt,
				ExpiresAt:   time.Unix(sessionData.ExpiresAt, 0),
				IP:          c.IP(),
				UserAgent:   c.Get("User-Agent"),
				Sub:         sessionData.Sub,
				Username:    sessionData.Username,
				Email:       sessionData.Email,
				FirstName:   sessionData.FirstName,
				LastName:    sessionData.LastName,
				DisplayName: sessionData.DisplayName,
				Picture:     pic,
				ProfileUrl:  pu,
				Groups:      sessionData.Groups,
				IsAdmin:     sessionData.IsAdmin,
			})
		}

		returnTo := stateCookie.ReturnTo
		if returnTo == "" {
			returnTo = "/"
		}
		return c.Redirect().Status(fiber.StatusFound).To(returnTo)
	}).Name(SessionLoginCallbackRoute)

	router.Get("/logout", func(c fiber.Ctx) error {
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

		return c.Redirect().Status(fiber.StatusFound).To(endSessionURL)
	}).Name(SessionLogoutRoute)

	router.Get("/logout/callback", func(c fiber.Ctx) error {
		loginURL, err := c.GetRouteURL(SessionLoginRoute, fiber.Map{})
		if err != nil {
			return err
		}
		return c.Redirect().Status(fiber.StatusFound).To(loginURL)
	}).Name(SessionLogoutCallbackRoute)
}

func redirectToLogin(c fiber.Ctx) error {
	loginURL, err := c.GetRouteURL(SessionLoginRoute, fiber.Map{})
	if err != nil {
		return err
	}

	u, err := url.Parse(loginURL)
	if err == nil {
		q := u.Query()
		q.Set("rd", c.Path())
		u.RawQuery = q.Encode()
		loginURL = u.String()
	}

	// HTMX requests need HX-Redirect so the browser performs a full navigation
	// instead of swapping the login page HTML into the current target element.
	// Use rd=/ — the current path is an API endpoint, not a page to return to.
	if c.Get("HX-Request") == "true" {
		htmxLoginURL, err := c.GetRouteURL(SessionLoginRoute, fiber.Map{})
		if err != nil {
			return err
		}
		c.Set("HX-Redirect", htmxLoginURL+"?rd=/")
		return c.SendStatus(fiber.StatusNoContent)
	}

	return c.Redirect().Status(fiber.StatusFound).To(loginURL)
}
