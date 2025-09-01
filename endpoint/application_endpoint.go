package endpoint

import (
	"dash/domain/model"
	"dash/domain/usecase"
	"dash/middleware"
	"dash/templ/components"
	"dash/templ/partials"
	"net/url"
	"strconv"
	"strings"

	"github.com/gofiber/fiber/v2"
	"github.com/samber/lo"
)

const (
	ApplicationsRoute            = "ApplicationsRoute"
	ApplicationsEditRoute        = "ApplicationsEditRoute"
	ApplicationCreateRoute       = "ApplicationCreateRoute"
	ApplicationUpdateRoute       = "ApplicationUpdateRoute"
	ApplicationDeleteRoute       = "ApplicationDeleteRoute"
	ApplicationsModalCreateRoute = "ApplicationsModalCreateRoute"
	ApplicationsModalEditRoute   = "ApplicationsModalEditRoute"
	ApplicationsModalDeleteRoute = "ApplicationsModalDeleteRoute"
)

type ApplicationDeps struct {
	App                   *fiber.App
	GetUserApplications   *usecase.GetUserApplications
	ListApplications      *usecase.ListApplications
	GetApplication        *usecase.GetApplication
	CreateApplication     *usecase.CreateApplication
	DeleteApplication     *usecase.DeleteApplication
	UpdateApplication     *usecase.UpdateApplication
	GetAvailableIconTypes *usecase.GetAvailableIconTypes
}

func Application(deps ApplicationDeps) {
	router := deps.App.
		Group("/applications").
		Use(middleware.GetUserFromIdToken)

	router.
		Use(middleware.HtmxOnly).
		Get("/", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			apps, err := deps.GetUserApplications.Execute(c.Context(), user.Groups)
			if err != nil {
				return err
			}

			inputs := lo.Map(apps, func(app model.AppLink, _ int) partials.ApplicationsInput {
				return partials.ApplicationsInput{
					ID:  app.ID,
					Url: app.Url,
					IconType: func() string {
						parts := strings.Split(app.Icon, ":")
						return parts[0]
					}(),
					Icon: func() string {
						parts := strings.Split(app.Icon, ":")
						return parts[1]
					}(),
					DisplayName: app.DisplayName,
					Domain: func() string {
						appUrl, _ := url.Parse(app.Url)
						return appUrl.Host
					}(),
				}
			})
			return middleware.Render(c, partials.Applications(inputs))
		}).Name(ApplicationsRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/edit", func(c *fiber.Ctx) error {
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

			inputs := lo.Map(apps, func(app model.AppLink, _ int) partials.ApplicationsEditInput {
				return partials.ApplicationsEditInput{
					ID: app.ID,
					IconType: func() string {
						parts := strings.Split(app.Icon, ":")
						return parts[0]
					}(),
					Icon: func() string {
						parts := strings.Split(app.Icon, ":")
						return parts[1]
					}(),
					DisplayName: app.DisplayName,
					Domain: func() string {
						appUrl, _ := url.Parse(app.Url)
						return appUrl.Host
					}(),
				}
			})
			return middleware.Render(c, partials.ApplicationsEdit(inputs))
		}).Name(ApplicationsEditRoute)

	router.
		Use(middleware.HtmxOnly).
		Post("/", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}
			if !user.IsAdmin {
				return fiber.NewError(fiber.StatusForbidden, "forbidden")
			}

			var body struct {
				IconType        string `form:"icon_type"`
				IconName        string `form:"icon_name"`
				DisplayName     string `form:"display_name"`
				Url             string `form:"url"`
				VisibleToGroups string `form:"visible_to_groups"`
			}
			if err := c.BodyParser(&body); err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid body")
			}

			if err := deps.CreateApplication.Execute(c.Context(), usecase.CreateApplicationInput{
				Icon: func() string {
					return body.IconType + ":" + body.IconName
				}(),
				DisplayName: body.DisplayName,
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

			return middleware.Render(c, partials.ModalCloseReload(partials.ModalCloseReloadInput{
				Trigger: partials.ModalCloseReloadApps,
			}))
		}).Name(ApplicationCreateRoute)

	router.
		Use(middleware.HtmxOnly).
		Put(":id", func(c *fiber.Ctx) error {
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
				IconType        string `form:"icon_type"`
				IconName        string `form:"icon_name"`
				DisplayName     string `form:"display_name"`
				Url             string `form:"url"`
				VisibleToGroups string `form:"visible_to_groups"`
			}
			if err := c.BodyParser(&body); err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid body")
			}

			if err := deps.UpdateApplication.Execute(c.Context(), usecase.UpdateApplicationInput{
				ID: uint(id64),
				Icon: func() string {
					return body.IconType + ":" + body.IconName
				}(),
				DisplayName: body.DisplayName,
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

			return middleware.Render(c, partials.ModalCloseReload(partials.ModalCloseReloadInput{
				Trigger: partials.ModalCloseReloadApps,
			}))
		}).Name(ApplicationUpdateRoute)

	router.
		Use(middleware.HtmxOnly).
		Delete(":id", func(c *fiber.Ctx) error {
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

			return middleware.Render(c, partials.ModalCloseReload(partials.ModalCloseReloadInput{
				Trigger: partials.ModalCloseReloadApps,
			}))
		}).Name(ApplicationDeleteRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/modal/create", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}
			if !user.IsAdmin {
				return fiber.NewError(fiber.StatusForbidden, "forbidden")
			}

			return middleware.Render(c, partials.ApplicationsCreateModal(partials.ApplicationsCreateModalInput{
				IconTypes: func() components.ModalUpserInputIconTypes {
					list, _ := deps.GetAvailableIconTypes.Execute(c.Context())
					return list
				}(),
			}))
		}).Name(ApplicationsModalCreateRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/modal/edit/:id", func(c *fiber.Ctx) error {
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

			return middleware.Render(c, partials.ApplicationsEditModal(partials.ApplicationsEditModalInput{
				ID: app.ID,
				IconTypes: func() components.ModalUpserInputIconTypes {
					list, _ := deps.GetAvailableIconTypes.Execute(c.Context())
					return list
				}(),
				Icon: func() components.ModalUpsertInputIcon {
					parts := strings.Split(app.Icon, ":")
					return components.ModalUpsertInputIcon{
						Type: parts[0],
						Name: parts[1],
					}
				}(),
				DisplayName:     app.DisplayName,
				Url:             app.Url,
				VisibleToGroups: strings.Join(app.VisibleToGroups, " "),
			}))
		}).Name(ApplicationsModalEditRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/modal/delete/:id", func(c *fiber.Ctx) error {
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

			return middleware.Render(c, partials.ApplicationsDeleteModal(partials.ApppplicationsDeleteModalInput{
				ID:          app.ID,
				DisplayName: app.DisplayName,
			}))
		}).Name(ApplicationsModalDeleteRoute)
}
