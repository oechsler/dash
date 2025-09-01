package endpoint

import (
	"dash/domain/model"
	"dash/domain/usecase"
	"dash/middleware"
	"net/url"
	"strconv"
	"strings"

	"github.com/gofiber/fiber/v2"
	"github.com/samber/lo"
)

type ApplicationDeps struct {
	App                 *fiber.App
	GetUserApplications *usecase.GetUserApplications
	ListApplications    *usecase.ListApplications
	GetApplication      *usecase.GetApplication
	CreateApplication   *usecase.CreateApplication
	DeleteApplication   *usecase.DeleteApplication
	UpdateApplication   *usecase.UpdateApplication
}

func Application(deps ApplicationDeps) {
	router := deps.App.
		Group("/applications").
		Use(middleware.GetUserFromIdToken)

	router.Get("/", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		apps, err := deps.GetUserApplications.Execute(c.Context(), user.Groups)
		if err != nil {
			return err
		}

		return c.Render("partials/applications", lo.Map(apps, func(app model.AppLink, _ int) fiber.Map {
			return fiber.Map{
				"id": app.ID,
				"icon": func() string {
					after, _ := strings.CutPrefix(app.Icon, "mdi:")
					return after
				}(),
				"display_name": app.DisplayName,
				"description":  app.Description,
				"url":          app.Url,
				"domain": func() string {
					appUrl, _ := url.Parse(app.Url)
					return appUrl.Host
				}(),
			}
		}))
	})

	router.Get("/edit", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}
		if !user.IsAdmin {
			return fiber.NewError(fiber.StatusForbidden, "forbidden")
		}

		apps, err := deps.ListApplications.Execute(c.Context())
		if err != nil {
			return err
		}

		return c.Render("partials/applications-edit", lo.Map(apps, func(app model.AppLink, _ int) fiber.Map {
			return fiber.Map{
				"id": app.ID,
				"icon": func() string {
					after, _ := strings.CutPrefix(app.Icon, "mdi:")
					return after
				}(),
				"display_name": app.DisplayName,
				"description":  app.Description,
				"url":          app.Url,
				"domain": func() string {
					appUrl, _ := url.Parse(app.Url)
					return appUrl.Host
				}(),
			}
		}))
	})

	router.Post("/", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}
		if !user.IsAdmin {
			return fiber.NewError(fiber.StatusForbidden, "forbidden")
		}

		var body struct {
			Icon            string `json:"icon" form:"icon"`
			DisplayName     string `json:"display_name" form:"display_name"`
			Description     string `json:"description" form:"description"`
			Url             string `json:"url" form:"url"`
			VisibleToGroups string `json:"visible_to_groups" form:"visible_to_groups"`
		}
		if err := c.BodyParser(&body); err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid body")
		}

		if err := deps.CreateApplication.Execute(c.Context(), usecase.CreateApplicationInput{
			Icon:        body.Icon,
			DisplayName: body.DisplayName,
			Description: &body.Description,
			Url:         body.Url,
			VisibleToGroups: func() []string {
				if body.VisibleToGroups == "" {
					return nil
				}
				return strings.Split(body.VisibleToGroups, " ")
			}(),
		}); err != nil {
			return err
		}

		return c.Render("partials/modal-reload", fiber.Map{
			"trigger": "apps-list",
		})
	})

	router.Put(":id", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}
		if !user.IsAdmin {
			return fiber.NewError(fiber.StatusForbidden, "forbidden")
		}

		id64, err := strconv.ParseUint(c.Params("id"), 10, 64)
		if err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid id")
		}

		var body struct {
			Icon            string `json:"icon" form:"icon"`
			DisplayName     string `json:"display_name" form:"display_name"`
			Description     string `json:"description" form:"description"`
			Url             string `json:"url" form:"url"`
			VisibleToGroups string `json:"visible_to_groups" form:"visible_to_groups"`
		}
		if err := c.BodyParser(&body); err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid body")
		}

		if err := deps.UpdateApplication.Execute(c.Context(), usecase.UpdateApplicationInput{
			ID:          uint(id64),
			Icon:        body.Icon,
			DisplayName: body.DisplayName,
			Description: func() *string {
				if body.Description == "" {
					return nil
				}
				return &body.Description
			}(),
			Url: body.Url,
			VisibleToGroups: func() []string {
				if body.VisibleToGroups == "" {
					return nil
				}
				return strings.Split(body.VisibleToGroups, " ")
			}(),
		}); err != nil {
			return err
		}
		return c.Render("partials/modal-reload", fiber.Map{
			"trigger": "apps-list",
		})
	})

	router.Delete(":id", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}
		if !user.IsAdmin {
			return fiber.NewError(fiber.StatusForbidden, "forbidden")
		}

		id64, err := strconv.ParseUint(c.Params("id"), 10, 64)
		if err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid id")
		}

		if err := deps.DeleteApplication.Execute(c.Context(), uint(id64)); err != nil {
			return err
		}

		return c.Render("partials/modal-reload", fiber.Map{
			"trigger": "apps-list",
		})
	})

	router.Get("/modal/create", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}
		if !user.IsAdmin {
			return fiber.NewError(fiber.StatusForbidden, "forbidden")
		}
		return c.Render("partials/modal-create-application", nil)
	})

	router.Get("/modal/delete/:id", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}
		if !user.IsAdmin {
			return fiber.NewError(fiber.StatusForbidden, "forbidden")
		}

		id64, err := strconv.ParseUint(c.Params("id"), 10, 64)
		if err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid id")
		}

		app, err := deps.GetApplication.Execute(c.Context(), uint(id64))
		if err != nil {
			return err
		}
		if app == nil {
			return fiber.NewError(fiber.StatusNotFound, "application not found")
		}

		return c.Render("partials/modal-delete-application", fiber.Map{
			"id":           app.ID,
			"display_name": app.DisplayName,
		})
	})

	router.Get("/modal/edit/:id", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}
		if !user.IsAdmin {
			return fiber.NewError(fiber.StatusForbidden, "forbidden")
		}

		id64, err := strconv.ParseUint(c.Params("id"), 10, 64)
		if err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid id")
		}

		app, err := deps.GetApplication.Execute(c.Context(), uint(id64))
		if err != nil {
			return err
		}
		if app == nil {
			return fiber.NewError(fiber.StatusNotFound, "application not found")
		}

		return c.Render("partials/modal-edit-application", fiber.Map{
			"id":           app.ID,
			"icon":         app.Icon,
			"display_name": app.DisplayName,
			"description": func() string {
				if app.Description == nil {
					return ""
				}
				return *app.Description
			}(),
			"url":               app.Url,
			"visible_to_groups": strings.Join(app.VisibleToGroups, " "),
		})
	})
}
