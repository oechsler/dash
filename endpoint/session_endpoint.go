package endpoint

import (
	"dash/environment"
	"net/url"
	"strings"

	"github.com/gofiber/fiber/v2"
)

const (
	SessionLoginRoute          = "SessionLoginRoute"
	SessionLoginCallbackRoute  = "SessionLoginCallbackRoute"
	SessionLogoutRoute         = "SessionLogoutRoute"
	SessionLogoutCallbackRoute = "SessionLogoutCallbackRoute"
)

func Session(
	env *environment.Env,
	app *fiber.App,
) {
	router := app.Group("/session")

	router.Get("/login", func(c *fiber.Ctx) error {
		singInUrl := c.BaseURL() + "/oauth2/sign_in"
		u, err := url.Parse(singInUrl)
		if err != nil {
			return err
		}

		redirectUri := c.Query("rd", "")

		q := u.Query()
		q.Add("rd", c.BaseURL()+redirectUri)
		u.RawQuery = q.Encode()

		return c.Redirect(u.String(), fiber.StatusFound)
	}).Name(SessionLoginRoute)

	router.Get("/login/callback", func(c *fiber.Ctx) error {
		callbackUrl := c.BaseURL() + "/oauth2/callback"
		u, err := url.Parse(callbackUrl)
		if err != nil {
			return err
		}

		q := u.Query()
		for key, values := range c.Queries() {
			q.Add(key, values)
		}
		u.RawQuery = q.Encode()

		return c.Redirect(u.String(), fiber.StatusFound)
	}).Name(SessionLoginCallbackRoute)

	router.Get("/logout", func(c *fiber.Ctx) error {
		if !c.QueryBool("from_provider", false) {
			u, err := url.Parse(c.BaseURL() + "/oauth2/sign_out")
			if err != nil {
				return err
			}

			q := u.Query()
			q.Add("rd", c.BaseURL()+"/session/login")
			u.RawQuery = q.Encode()

			return c.Redirect(u.String(), fiber.StatusFound)
		}

		authorizationHeader := c.Get("Authorization")
		idToken, _ := strings.CutPrefix(authorizationHeader, "Bearer ")

		endSessionUrlString := env.String("OAUTH2_END_SESSION_URL", "")
		endSessionUrl, err := url.Parse(endSessionUrlString)
		if err != nil {
			signOutUrl, err := url.Parse(c.BaseURL() + "/oauth2/sign_out")
			if err != nil {
				return err
			}
			return c.Redirect(signOutUrl.String(), fiber.StatusFound)
		}

		sessionLogoutCallbackRouteUrl, err := c.GetRouteURL(SessionLogoutCallbackRoute, fiber.Map{})
		if err != nil {
			return err
		}

		q := endSessionUrl.Query()
		q.Add("id_token_hint", idToken)
		q.Add("post_logout_redirect_uri", c.BaseURL()+sessionLogoutCallbackRouteUrl)
		endSessionUrl.RawQuery = q.Encode()

		signOutUrl, err := url.Parse(c.BaseURL() + "/oauth2/sign_out")
		if err != nil {
			return err
		}

		q = signOutUrl.Query()
		q.Add("rd", endSessionUrl.String())
		signOutUrl.RawQuery = q.Encode()

		return c.Redirect(signOutUrl.String(), fiber.StatusFound)
	}).Name(SessionLogoutRoute)

	router.Get("/logout/callback", func(c *fiber.Ctx) error {
		sessionLoginRouteUrl, err := c.GetRouteURL(SessionLoginRoute, fiber.Map{})
		if err != nil {
			return err
		}

		return c.Redirect(c.BaseURL()+sessionLoginRouteUrl, fiber.StatusFound)
	}).Name(SessionLogoutCallbackRoute)
}

func redirectToLogin(c *fiber.Ctx) error {
	sessionLoginRouteUrl, err := c.GetRouteURL(SessionLoginRoute, fiber.Map{})
	if err != nil {
		return err
	}
	return c.Redirect(sessionLoginRouteUrl, fiber.StatusFound)
}
