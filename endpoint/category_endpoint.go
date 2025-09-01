package endpoint

import (
	"dash/domain/model"
	"dash/domain/usecase"
	"dash/middleware"
	"errors"
	"net/url"
	"strconv"
	"strings"

	"github.com/gofiber/fiber/v2"
	"github.com/samber/lo"
)

const (
	CategoriesRoute     = "CategoriesRoute"
	CategoryCreateRoute = "CategoryCreateRoute"
	CategoryUpdateRoute = "CategoryUpdateRoute"
	CategoryDeleteRoute = "CategoryDeleteRoute"
)

type CategoryDeps struct {
	App                      *fiber.App
	GetUserCategories        *usecase.GetUserCategories
	GetUserShelvedCategories *usecase.GetUserShelvedCategories
	GetUserCategory          *usecase.GetUserCategory
	CategoryCreate           *usecase.CreateUserCategory
	CategoryUpdate           *usecase.UpdateUserCategory
	CategoryDelete           *usecase.DeleteUserCategory
}

func Category(deps CategoryDeps) {
	router := deps.App.
		Group("/categories").
		Use(middleware.GetUserFromIdToken)

	router.Get("/", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		categories, err := deps.GetUserCategories.Execute(c.Context(), user.ID)
		if err != nil {
			return err
		}

		return c.Render("partials/categories", lo.Map(categories, func(category model.Category, _ int) fiber.Map {
			return fiber.Map{
				"id":           category.ID,
				"display_name": category.DisplayName,
				"bookmarks": lo.Map(category.Bookmarks, func(bookmark model.Bookmark, _ int) fiber.Map {
					return fiber.Map{
						"id": bookmark.ID,
						"icon": func() string {
							after, _ := strings.CutPrefix(bookmark.Icon, "mdi:")
							return after
						}(),
						"display_name": bookmark.DisplayName,
						"description":  bookmark.Description,
						"url":          bookmark.Url,
						"domain": func() string {
							bookmarkUrl, _ := url.Parse(bookmark.Url)
							return bookmarkUrl.Host
						}(),
					}
				}),
			}
		}))
	}).Name(CategoriesRoute)

	router.Get("/edit", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		categories, err := deps.GetUserCategories.Execute(c.Context(), user.ID)
		if err != nil {
			return err
		}

		return c.Render("partials/categories-edit", lo.Map(categories, func(category model.Category, _ int) fiber.Map {
			return fiber.Map{
				"id":           category.ID,
				"display_name": category.DisplayName,
				"bookmarks": lo.Map(category.Bookmarks, func(bookmark model.Bookmark, _ int) fiber.Map {
					return fiber.Map{
						"id": bookmark.ID,
						"icon": func() string {
							after, _ := strings.CutPrefix(bookmark.Icon, "mdi:")
							return after
						}(),
						"display_name": bookmark.DisplayName,
						"description":  bookmark.Description,
						"url":          bookmark.Url,
					}
				}),
			}
		}))
	})

	router.Post("/", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		var body struct {
			DisplayName string `json:"display_name" form:"display_name"`
			IsShelved   bool   `json:"is_shelved" form:"is_shelved"`
		}
		if err := c.BodyParser(&body); err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid body")
		}

		if err := deps.CategoryCreate.Execute(c.Context(), user.ID, usecase.CreateUserCategoryInput{
			DisplayName: body.DisplayName,
			IsShelved:   body.IsShelved,
		}); err != nil {
			if errors.Is(err, usecase.ErrValidation) {
				return fiber.NewError(fiber.StatusBadRequest, err.Error())
			}
			if errors.Is(err, usecase.ErrDashboardNotFound) {
				return fiber.NewError(fiber.StatusNotFound, "dashboard not found")
			}
			if errors.Is(err, usecase.ErrUserDoesNotOwnDashboard) {
				return fiber.NewError(fiber.StatusForbidden, "user does not own dashboard")
			}
			return err
		}

		// TODO: To be RESTful, add location header
		//  that points to the newly created category

		return c.Render("partials/modal-reload", fiber.Map{
			"trigger": "categories-list",
		})
	}).Name(CategoryCreateRoute)

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
			DisplayName string `json:"display_name" form:"display_name"`
			IsShelved   bool   `json:"is_shelved" form:"is_shelved"`
		}
		if err := c.BodyParser(&body); err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid body")
		}

		if err := deps.CategoryUpdate.Execute(c.Context(), user.ID, usecase.UpdateUserCategoryInput{
			ID:          uint(id64),
			DisplayName: body.DisplayName,
			IsShelved:   body.IsShelved,
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
	}).Name(CategoryUpdateRoute)

	router.Delete(":id", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		id64, err := strconv.ParseUint(c.Params("id"), 10, 64)
		if err != nil {
			return fiber.NewError(fiber.StatusBadRequest, "invalid id")
		}

		if err := deps.CategoryDelete.Execute(c.Context(), user.ID, uint(id64)); err != nil {
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
	}).Name(CategoryDeleteRoute)

	router.Get("/shelved", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		shelved, err := deps.GetUserShelvedCategories.Execute(c.Context(), user.ID)
		if err != nil {
			return err
		}

		return c.Render("partials/categories-shelved", lo.Map(shelved, func(category model.Category, _ int) fiber.Map {
			return fiber.Map{
				"id":           category.ID,
				"display_name": category.DisplayName,
				"bookmarks": lo.Map(category.Bookmarks, func(bookmark model.Bookmark, _ int) fiber.Map {
					return fiber.Map{
						"id": bookmark.ID,
						"icon": func() string {
							after, _ := strings.CutPrefix(bookmark.Icon, "mdi:")
							return after
						}(),
						"display_name": bookmark.DisplayName,
						"description":  bookmark.Description,
						"url":          bookmark.Url,
						"domain": func() string {
							bookmarkUrl, _ := url.Parse(bookmark.Url)
							return bookmarkUrl.Host
						}(),
					}
				}),
			}
		}))
	})

	router.Get("/shelved/edit", func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		shelved, err := deps.GetUserShelvedCategories.Execute(c.Context(), user.ID)
		if err != nil {
			return err
		}

		return c.Render("partials/categories-shelved-edit", lo.Map(shelved, func(category model.Category, _ int) fiber.Map {
			return fiber.Map{
				"id":           category.ID,
				"display_name": category.DisplayName,
				"bookmarks": lo.Map(category.Bookmarks, func(bookmark model.Bookmark, _ int) fiber.Map {
					return fiber.Map{
						"id": bookmark.ID,
						"icon": func() string {
							after, _ := strings.CutPrefix(bookmark.Icon, "mdi:")
							return after
						}(),
						"display_name": bookmark.DisplayName,
						"description":  bookmark.Description,
						"url":          bookmark.Url,
						"domain": func() string {
							bookmarkUrl, _ := url.Parse(bookmark.Url)
							return bookmarkUrl.Host
						}(),
					}
				}),
			}
		}))
	})

	router.Get("/modal/create", func(c *fiber.Ctx) error {
		return c.Render("partials/modal-create-category", nil)
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

		return c.Render("partials/modal-edit-category", fiber.Map{
			"id":           category.ID,
			"display_name": category.DisplayName,
			"is_shelved":   category.IsShelved,
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

		return c.Render("partials/modal-delete-category", fiber.Map{
			"id":           category.ID,
			"display_name": category.DisplayName,
		})
	})

	router.Get("/modal/shelved/:isShelved", func(c *fiber.Ctx) error {
		_, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return redirectToLogin(c)
		}

		return c.Render("partials/modal-shelved-category", fiber.Map{
			"is_shelved": c.Params("isShelved") == "true",
		})
	})
}
