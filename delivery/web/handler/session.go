package handler

import (
	"net/url"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	"git.at.oechsler.it/samuel/dash/v2/infra/oidc"

	"github.com/gofiber/fiber/v3"
)

const (
	SessionLoginRoute          = "SessionLoginRoute"
	SessionLoginCallbackRoute  = "SessionLoginCallbackRoute"
	SessionLogoutRoute         = "SessionLogoutRoute"
	SessionLogoutCallbackRoute = "SessionLogoutCallbackRoute"
	SessionRefreshRoute        = "SessionRefreshRoute"
)

func Session(
	app *fiber.App,
	provider *oidc.Provider,
	store *oidc.SessionStore,
	createSession command.SessionCreator,
	refreshSession command.SessionRefresher,
	terminateSession command.SessionTerminator,
	migrateUserID command.UserIDMigrator,
	resolveOrCreateUser command.UserResolver,
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

	// Refresh: re-authenticates via OIDC to update identity on the current pinned session.
	// The existing session record is updated in-place — no new SessionID is issued.
	router.Get("/refresh", func(c fiber.Ctx) error {
		sessionData, ok := store.LoadExpired(c)
		if !ok || sessionData.SessionID == "" {
			return redirectToLogin(c)
		}

		state, err := oidc.GenerateState()
		if err != nil {
			return err
		}
		codeVerifier, err := oidc.GenerateCodeVerifier()
		if err != nil {
			return err
		}

		if err := store.SaveStateCookie(c, oidc.StateCookie{
			State:            state,
			CodeVerifier:     codeVerifier,
			ReturnTo:         c.Query("rd", "/"),
			RefreshSessionID: sessionData.SessionID,
		}); err != nil {
			return err
		}

		return c.Redirect().Status(fiber.StatusFound).To(provider.BeginAuth(state, codeVerifier))
	}).Name(SessionRefreshRoute)

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

		// Resolve the stable internal UserID via the IdP links table.
		if resolveOrCreateUser != nil {
			if userID, isNew, err := resolveOrCreateUser.Handle(c.Context(), provider.Issuer(), idToken.Subject); err == nil {
				identity.UserID = userID
				if isNew && migrateUserID != nil {
					// First login after the UUID-based UserID scheme was introduced.
					// Move any pre-existing data from sub and the legacy preferred_username
					// fallback ID to the new stable UUID.
					// TODO(v3): remove the legacy migration once all deployments have
					// gone through at least one login cycle after this upgrade.
					_ = migrateUserID.Handle(c.Context(), idToken.Subject, userID)
					if legacyID, err := provider.LegacyUserID(idToken); err == nil {
						_ = migrateUserID.Handle(c.Context(), legacyID, userID)
					}
				}
			}
		}

		if stateCookie.RefreshSessionID != "" {
			// Resolve UserID also on refresh so the stored identity stays consistent.
			if resolveOrCreateUser != nil {
				if userID, _, err := resolveOrCreateUser.Handle(c.Context(), provider.Issuer(), idToken.Subject); err == nil {
					identity.UserID = userID
				}
			}
			// Refresh flow: update identity and token timing on the existing session record.
			if refreshSession != nil {
				_ = refreshSession.Handle(c.Context(), command.RefreshSessionCmd{
					SessionID:   stateCookie.RefreshSessionID,
					Sub:         idToken.Subject,
					Username:    identity.Username,
					Email:       identity.Email,
					FirstName:   identity.FirstName,
					LastName:    identity.LastName,
					DisplayName: identity.DisplayName,
					Picture:     ptrStr(identity.Picture),
					ProfileUrl:  ptrStr(identity.ProfileUrl),
					Groups:      identity.Groups,
					IsAdmin:     identity.IsAdmin,
					IssuedAt:    idToken.IssuedAt,
					ExpiresAt:   idToken.Expiry,
				})
			}
			if _, err := store.SaveWithID(c, rawIDToken, stateCookie.RefreshSessionID, true); err != nil {
				return err
			}

			returnTo := stateCookie.ReturnTo
			if returnTo == "" {
				returnTo = "/"
			}
			return c.Redirect().Status(fiber.StatusFound).To(returnTo)
		}

		sessionData, err := store.Save(c, rawIDToken)
		if err != nil {
			return err
		}

		// Persist the session to the DB so it appears in the session overview.
		// Ignore errors — a DB hiccup must not prevent login.
		if createSession != nil {
			_ = createSession.Handle(c.Context(), command.CreateSessionCmd{
				SessionID:   sessionData.SessionID,
				UserID:      identity.UserID,
				Sub:         idToken.Subject,
				Username:    identity.Username,
				Email:       identity.Email,
				FirstName:   identity.FirstName,
				LastName:    identity.LastName,
				DisplayName: identity.DisplayName,
				Picture:     ptrStr(identity.Picture),
				ProfileUrl:  ptrStr(identity.ProfileUrl),
				Groups:      identity.Groups,
				IsAdmin:     identity.IsAdmin,
				IssuedAt:    idToken.IssuedAt,
				ExpiresAt:   idToken.Expiry,
				IP:          c.IP(),
				UserAgent:   c.Get("User-Agent"),
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

		// Delete the DB record so the session disappears from the overview
		// and the revocation check denies any lingering cookie on other tabs.
		if ok && sessionData.SessionID != "" && terminateSession != nil {
			_ = terminateSession.Handle(c.Context(), sessionData.SessionID)
		}

		// Use id_token_hint for OIDC end-session when the raw token was stored in
		// the cookie. For users with very many group memberships the token may have
		// been omitted (cookie size budget); those users get local-only logout.
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

// ptrStr dereferences a *string, returning "" for nil.
func ptrStr(p *string) string {
	if p == nil {
		return ""
	}
	return *p
}
