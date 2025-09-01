package endpoint

import (
	"dash/domain/usecase"
	"dash/middleware"
	"dash/templ/components"
	"dash/templ/partials"
	"errors"
	"strconv"
	"strings"

	"github.com/gofiber/fiber/v2"
)

const (
	BookmarkCreateRoute      = "BookmarkCreateRoute"
	BookmarkUpdateRoute      = "BookmarkUpdateRoute"
	BookmarkDeleteRoute      = "BookmarkDeleteRoute"
	BookmarkModalCreateRoute = "BookmarkModalCreateRoute"
	BookmarkModalEditRoute   = "BookmarkModalEditRoute"
	BookmarkModalDeleteRoute = "BookmarkModalDeleteRoute"
)

type BookmarkDeps struct {
	App                   *fiber.App
	GetUserCategory       *usecase.GetUserCategory
	GetUserBookmark       *usecase.GetUserBookmark
	BookmarkCreate        *usecase.CreateUserBookmark
	BookmarkUpdate        *usecase.UpdateUserBookmark
	BookmarkDelete        *usecase.DeleteUserBookmark
	GetAvailableIconTypes *usecase.GetAvailableIconTypes
}

func Bookmark(deps BookmarkDeps) {
	router := deps.App.
		Group("/bookmarks").
		Use(middleware.GetUserFromIdToken)

	router.
		Use(middleware.HtmxOnly).
		Post("/", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			var body struct {
				IconName    string `form:"icon_name"`
				IconType    string `form:"icon_type"`
				DisplayName string `form:"display_name"`
				Url         string `form:"url"`
				CategoryID  uint   `form:"category_id"`
			}
			if err := c.BodyParser(&body); err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid body")
			}

			if err := deps.BookmarkCreate.Execute(c.Context(), user.ID, usecase.CreateUserBookmarkInput{
				Icon: func() string {
					return body.IconType + ":" + body.IconName
				}(),
				DisplayName: body.DisplayName,
				Url:         body.Url,
				CategoryID:  body.CategoryID,
			}); err != nil {
				if errors.Is(err, usecase.ErrValidation) {
					return fiber.NewError(fiber.StatusBadRequest, err.Error())
				}
				if errors.Is(err, usecase.ErrCategoryNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "category not found")
				}
				if errors.Is(err, usecase.ErrDashboardNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "dashboard not found")
				}
				if errors.Is(err, usecase.ErrUserDoesNotOwnDashboard) {
					return fiber.NewError(fiber.StatusForbidden, "user does not own dashboard")
				}
				return err
			}

			return middleware.Render(c, partials.ModalCloseReload(partials.ModalCloseReloadInput{
				Trigger: partials.ModalCloseReloadCategories,
			}))
		}).Name(BookmarkCreateRoute)

	router.
		Use(middleware.HtmxOnly).
		Put(":id", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			id64, err := strconv.ParseUint(c.Params("id"), 10, 64)
			if err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid id")
			}

			var body struct {
				IconName    string `form:"icon_name"`
				IconType    string `form:"icon_type"`
				DisplayName string `form:"display_name"`
				Url         string `form:"url"`
				CategoryID  uint   `form:"category_id"`
			}
			if err := c.BodyParser(&body); err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid body")
			}

			if err := deps.BookmarkUpdate.Execute(c.Context(), user.ID, usecase.UpdateUserBookmarkInput{
				ID: uint(id64),
				Icon: func() string {
					return body.IconType + ":" + body.IconName
				}(),
				DisplayName: body.DisplayName,
				Url:         body.Url,
				CategoryID:  body.CategoryID,
			}); err != nil {
				if errors.Is(err, usecase.ErrValidation) {
					return fiber.NewError(fiber.StatusBadRequest, err.Error())
				}
				if errors.Is(err, usecase.ErrCategoryNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "category not found")
				}
				if errors.Is(err, usecase.ErrDashboardNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "dashboard not found")
				}
				if errors.Is(err, usecase.ErrUserDoesNotOwnDashboard) {
					return fiber.NewError(fiber.StatusForbidden, "user does not own dashboard")
				}
				return err
			}

			return middleware.Render(c, partials.ModalCloseReload(partials.ModalCloseReloadInput{
				Trigger: partials.ModalCloseReloadCategories,
			}))
		}).Name(BookmarkUpdateRoute)

	router.
		Use(middleware.HtmxOnly).
		Delete(":id", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			id64, err := strconv.ParseUint(c.Params("id"), 10, 64)
			if err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid id")
			}

			if err := deps.BookmarkDelete.Execute(c.Context(), user.ID, uint(id64)); err != nil {
				if errors.Is(err, usecase.ErrValidation) {
					return fiber.NewError(fiber.StatusBadRequest, err.Error())
				}
				if errors.Is(err, usecase.ErrCategoryNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "category not found")
				}
				if errors.Is(err, usecase.ErrDashboardNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "dashboard not found")
				}
				if errors.Is(err, usecase.ErrUserDoesNotOwnDashboard) {
					return fiber.NewError(fiber.StatusForbidden, "user does not own dashboard")
				}
				return err
			}

			return middleware.Render(c, partials.ModalCloseReload(partials.ModalCloseReloadInput{
				Trigger: partials.ModalCloseReloadCategories,
			}))
		}).Name(BookmarkDeleteRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/modal/create/:categoryId", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			id64, err := strconv.ParseUint(c.Params("categoryId"), 10, 64)
			if err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid id")
			}

			category, err := deps.GetUserCategory.Execute(c.Context(), user.ID, uint(id64))
			if err != nil {
				if errors.Is(err, usecase.ErrCategoryNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "category not found")
				}
				if errors.Is(err, usecase.ErrDashboardNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "dashboard not found")
				}
				if errors.Is(err, usecase.ErrUserDoesNotOwnDashboard) {
					return fiber.NewError(fiber.StatusForbidden, "user does not own dashboard")
				}
				return err
			}

			return middleware.Render(c, partials.BookmarksCreateModal(partials.BookmarksCreateModalInput{
				CategoryID:          category.ID,
				CategoryDisplayName: category.DisplayName,
				IconTypes: func() components.ModalUpserInputIconTypes {
					list, _ := deps.GetAvailableIconTypes.Execute(c.Context())
					return list
				}(),
			}))
		}).Name(BookmarkModalCreateRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/modal/edit/:id", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			id64, err := strconv.ParseUint(c.Params("id"), 10, 64)
			if err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid id")
			}

			bookmark, err := deps.GetUserBookmark.Execute(c.Context(), user.ID, uint(id64))
			if err != nil {
				if errors.Is(err, usecase.ErrBookmarkNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "bookmark not found")
				}
				if errors.Is(err, usecase.ErrCategoryNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "category not found")
				}
				if errors.Is(err, usecase.ErrDashboardNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "dashboard not found")
				}
				if errors.Is(err, usecase.ErrUserDoesNotOwnDashboard) {
					return fiber.NewError(fiber.StatusForbidden, "user does not own dashboard")
				}
				return err
			}

			return middleware.Render(c, partials.BookmarksEditModal(partials.BookmarksEditModalInput{
				ID: bookmark.ID,
				IconTypes: func() components.ModalUpserInputIconTypes {
					list, _ := deps.GetAvailableIconTypes.Execute(c.Context())
					return components.ModalUpserInputIconTypes(list)
				}(),
				Icon: func() components.ModalUpsertInputIcon {
					parts := strings.Split(bookmark.Icon, ":")
					return components.ModalUpsertInputIcon{
						Type: parts[0],
						Name: parts[1],
					}
				}(),
				DisplayName: bookmark.DisplayName,
				Url:         bookmark.Url,
				CategoryID:  bookmark.CategoryID,
			}))
		}).Name(BookmarkModalEditRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/modal/delete/:id", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			id64, err := strconv.ParseUint(c.Params("id"), 10, 64)
			if err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid id")
			}

			bookmark, err := deps.GetUserBookmark.Execute(c.Context(), user.ID, uint(id64))
			if err != nil {
				if errors.Is(err, usecase.ErrBookmarkNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "bookmark not found")
				}
				if errors.Is(err, usecase.ErrCategoryNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "category not found")
				}
				if errors.Is(err, usecase.ErrDashboardNotFound) {
					return fiber.NewError(fiber.StatusNotFound, "dashboard not found")
				}
				if errors.Is(err, usecase.ErrUserDoesNotOwnDashboard) {
					return fiber.NewError(fiber.StatusForbidden, "user does not own dashboard")
				}
				return err
			}

			return middleware.Render(c, partials.BookmarksDeleteModal(partials.BookmarksDeleteModalInput{
				ID:          bookmark.ID,
				DisplayName: bookmark.DisplayName,
			}))
		}).Name(BookmarkModalDeleteRoute)
}
