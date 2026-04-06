package handler

import (
	"strconv"
	"strings"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	"git.at.oechsler.it/samuel/dash/v2/app/query"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/middleware"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/templ/components"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/templ/partials"
	"git.at.oechsler.it/samuel/dash/v2/domain/model"
	"git.at.oechsler.it/samuel/dash/v2/infra/oidc"

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
	SessionStore          *oidc.SessionStore
	App                   *fiber.App
	GetUserApplications   query.UserApplicationsGetter
	ListApplications      query.ApplicationsLister
	GetApplication        query.ApplicationGetter
	CreateApplication     command.ApplicationCreator
	DeleteApplication     command.ApplicationDeleter
	UpdateApplication     command.ApplicationUpdater
	GetAvailableIconTypes query.AvailableIconTypesGetter
}

func Application(deps ApplicationDeps) {
	router := deps.App.
		Group("/applications").
		Use(middleware.LoadUserFromSession(deps.SessionStore))

	router.
		Use(middleware.HtmxOnly).
		Get("/", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			apps, err := deps.GetUserApplications.Handle(c.Context(), user.Groups)
			if err != nil {
				return err
			}

			inputs := lo.Map(apps, func(app model.AppLink, _ int) partials.ApplicationsInput {
				return partials.ApplicationsInput{
					ID:          app.ID,
					Url:         app.Url.String(),
					IconType:    app.Icon.Type(),
					Icon:        app.Icon.Name(),
					DisplayName: app.DisplayName,
					Domain:      app.Url.Host(),
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

			apps, err := deps.ListApplications.Handle(c.Context())
			if err != nil {
				return err
			}

			inputs := lo.Map(apps, func(app model.AppLink, _ int) partials.ApplicationsEditInput {
				return partials.ApplicationsEditInput{
					ID:          app.ID,
					IconType:    app.Icon.Type(),
					Icon:        app.Icon.Name(),
					DisplayName: app.DisplayName,
					Domain:      app.Url.Host(),
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

			if err := deps.CreateApplication.Handle(c.Context(), command.CreateApplicationCmd{
				Icon:        body.IconType + ":" + body.IconName,
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

			if err := deps.UpdateApplication.Handle(c.Context(), command.UpdateApplicationCmd{
				ID:          uint(id64),
				Icon:        body.IconType + ":" + body.IconName,
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

			if err := deps.DeleteApplication.Handle(c.Context(), uint(id64)); err != nil {
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
					list, _ := deps.GetAvailableIconTypes.Handle(c.Context())
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

			app, err := deps.GetApplication.Handle(c.Context(), uint(id64))
			if err != nil {
				return httpError(err)
			}

			return middleware.Render(c, partials.ApplicationsEditModal(partials.ApplicationsEditModalInput{
				ID: app.ID,
				IconTypes: func() components.ModalUpserInputIconTypes {
					list, _ := deps.GetAvailableIconTypes.Handle(c.Context())
					return list
				}(),
				Icon: components.ModalUpsertInputIcon{
					Type: app.Icon.Type(),
					Name: app.Icon.Name(),
				},
				DisplayName:     app.DisplayName,
				Url:             app.Url.String(),
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

			app, err := deps.GetApplication.Handle(c.Context(), uint(id64))
			if err != nil {
				return httpError(err)
			}

			return middleware.Render(c, partials.ApplicationsDeleteModal(partials.ApppplicationsDeleteModalInput{
				ID:          app.ID,
				DisplayName: app.DisplayName,
			}))
		}).Name(ApplicationsModalDeleteRoute)
}
