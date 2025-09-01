package endpoint

import (
	"dash/domain/usecase"
	"dash/middleware"
	"errors"
	"strconv"

	"github.com/gofiber/fiber/v2"
)

const (
	BookmarksRoute      = "BookmarksRoute"
	BookmarkCreateRoute = "BookmarkCreateRoute"
	BookmarkUpdateRoute = "BookmarkUpdateRoute"
	BookmarkDeleteRoute = "BookmarkDeleteRoute"
)

type BookmarkDeps struct {
	App               *fiber.App
	GetUserBookmarks  *usecase.GetUserBookmarks
	GetUserBookmark   *usecase.GetUserBookmark
	GetUserCategories *usecase.GetUserCategories
	GetUserCategory   *usecase.GetUserCategory
	BookmarkCreate    *usecase.CreateUserBookmark
	BookmarkUpdate    *usecase.UpdateUserBookmark
	BookmarkDelete    *usecase.DeleteUserBookmark
}

func Bookmark(deps BookmarkDeps) {
	router := deps.App.
		Group("/bookmarks").
		Use(middleware.GetUserFromIdToken)

	router.Post("/", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		var body struct {
			Icon        string `json:"icon" form:"icon"`
			DisplayName string `json:"display_name" form:"display_name"`
			Description string `json:"description" form:"description"`
			Url         string `json:"url" form:"url"`
			CategoryID  uint   `json:"category_id" form:"category_id"`
		}
		if err := c.BodyParser(&body); err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid body")
		}

		if err := deps.BookmarkCreate.Execute(c.Context(), user.ID, usecase.CreateUserBookmarkInput{
			Icon:        body.Icon,
			DisplayName: body.DisplayName,
			Description: func() *string {
				if body.Description == "" {
					return nil
				}
				return &body.Description
			}(),
			Url:        body.Url,
			CategoryID: body.CategoryID,
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

		return c.Render("partials/modal-reload", fiber.Map{
			"trigger": "categories-list",
		})
	}).Name(BookmarkCreateRoute)

	router.Put(":id", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		id64, err := strconv.ParseUint(c.Params("id"), 10, 64)
		if err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid id")
		}

		var body struct {
			Icon        string `json:"icon" form:"icon"`
			DisplayName string `json:"display_name" form:"display_name"`
			Description string `json:"description" form:"description"`
			Url         string `json:"url" form:"url"`
			CategoryID  uint   `json:"category_id" form:"category_id"`
		}
		if err := c.BodyParser(&body); err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid body")
		}

		if err := deps.BookmarkUpdate.Execute(c.Context(), user.ID, usecase.UpdateUserBookmarkInput{
			ID:          uint(id64),
			Icon:        body.Icon,
			DisplayName: body.DisplayName,
			Description: func() *string {
				if body.Description == "" {
					return nil
				}
				return &body.Description
			}(),
			Url:        body.Url,
			CategoryID: body.CategoryID,
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

		return c.Render("partials/modal-reload", fiber.Map{
			"trigger": "categories-list",
		})
	}).Name(BookmarkUpdateRoute)

	router.Delete(":id", func(c *fiber.Ctx) error {
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

		return c.Render("partials/modal-reload", fiber.Map{
			"trigger": "categories-list",
		})
	}).Name(BookmarkDeleteRoute)

	router.Get("/modal/create/:categoryId", func(c *fiber.Ctx) error {
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
		
		return c.Render("partials/modal-create-bookmark", fiber.Map{
			"category_id":           category.ID,
			"category_display_name": category.DisplayName,
		})
	})

	router.Get("/modal/edit/:id", func(c *fiber.Ctx) error {
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

		return c.Render("partials/modal-edit-bookmark", fiber.Map{
			"id":           bookmark.ID,
			"icon":         bookmark.Icon,
			"display_name": bookmark.DisplayName,
			"description": func() string {
				if bookmark.Description == nil {
					return ""
				}
				return *bookmark.Description
			}(),
			"url":         bookmark.Url,
			"category_id": bookmark.CategoryID,
		})
	})

	router.Get("/modal/delete/:id", func(c *fiber.Ctx) error {
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
		
		return c.Render("partials/modal-delete-bookmark", fiber.Map{
			"id":           bookmark.ID,
			"display_name": bookmark.DisplayName,
		})
	})
}
