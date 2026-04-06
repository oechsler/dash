package handler

import (
	"strconv"

	"github.com/oechsler-it/dash/app/command"
	"github.com/oechsler-it/dash/app/query"
	"github.com/oechsler-it/dash/delivery/web/middleware"
	"github.com/oechsler-it/dash/delivery/web/templ/partials"
	"github.com/oechsler-it/dash/domain/model"
	"github.com/oechsler-it/dash/infra/oidc"

	"github.com/gofiber/fiber/v2"
	"github.com/samber/lo"
)

const (
	CategoriesRoute             = "CategoriesRoute"
	CategoriesEditRoute         = "CategoriesEditRoute"
	CategoriesShelvedRoute      = "CategoriesShelvedRoute"
	CategoriesShelvedEditRoute  = "CategoriesShelvedEditRoute"
	CategoriesModalCreateRoute  = "CategoriesModalCreateRoute"
	CategoriesModalEditRoute    = "CategoriesModalEditRoute"
	CategoriesModalDeleteRoute  = "CategoriesModalDeleteRoute"
	CategoriesModalShelvedRoute = "CategoriesModalShelvedRoute"
	CategoryCreateRoute         = "CategoryCreateRoute"
	CategoryUpdateRoute         = "CategoryUpdateRoute"
	CategoryDeleteRoute         = "CategoryDeleteRoute"
)

type CategoryDeps struct {
	SessionStore             *oidc.SessionStore
	App                      *fiber.App
	GetUserCategories        query.UserCategoriesGetter
	GetUserShelvedCategories query.UserShelvedCategoriesGetter
	GetUserCategory          query.UserCategoryGetter
	CategoryCreate           command.UserCategoryCreator
	CategoryUpdate           command.UserCategoryUpdater
	CategoryDelete           command.UserCategoryDeleter
}

func Category(deps CategoryDeps) {
	router := deps.App.
		Group("/categories").
		Use(middleware.LoadUserFromSession(deps.SessionStore))

	router.
		Use(middleware.HtmxOnly).
		Get("/", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			categories, err := deps.GetUserCategories.Handle(c.Context(), user.UserID)
			if err != nil {
				return err
			}

			inputs := lo.Map(categories, func(category model.Category, _ int) partials.CategoriesInput {
				return partials.CategoriesInput{
					ID:          category.ID,
					DisplayName: category.DisplayName,
					Bookmarks: lo.Map(
						category.Bookmarks,
						func(bookmark model.Bookmark, _ int) partials.CategoriesInputBookmark {
							return partials.CategoriesInputBookmark{
								ID:          bookmark.ID,
								IconType:    bookmark.Icon.Type(),
								Icon:        bookmark.Icon.Name(),
								DisplayName: bookmark.DisplayName,
								Url:         bookmark.Url.String(),
							}
						},
					),
				}
			})
			return middleware.Render(c, partials.Categories(inputs))
		}).Name(CategoriesRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/edit", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			categories, err := deps.GetUserCategories.Handle(c.Context(), user.UserID)
			if err != nil {
				return err
			}

			inputs := lo.Map(categories, func(category model.Category, _ int) partials.CategoriesEditInput {
				return partials.CategoriesEditInput{
					ID:          category.ID,
					DisplayName: category.DisplayName,
					Bookmarks: lo.Map(
						category.Bookmarks,
						func(bookmark model.Bookmark, _ int) partials.CategoriesEditInputBookmark {
							return partials.CategoriesEditInputBookmark{
								ID:          bookmark.ID,
								IconType:    bookmark.Icon.Type(),
								Icon:        bookmark.Icon.Name(),
								DisplayName: bookmark.DisplayName,
							}
						},
					),
				}
			})
			return middleware.Render(c, partials.CategoriesEdit(inputs))
		}).Name(CategoriesEditRoute)

	router.
		Use(middleware.HtmxOnly).
		Post("/", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			var body struct {
				DisplayName string `form:"display_name"`
				IsShelved   bool   `form:"is_shelved"`
			}
			if err := c.BodyParser(&body); err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid body")
			}

			if err := deps.CategoryCreate.Handle(c.Context(), user.UserID, command.CreateUserCategoryCmd{
				DisplayName: body.DisplayName,
				IsShelved:   body.IsShelved,
			}); err != nil {
				return httpError(err)
			}

			return middleware.Render(c, partials.ModalCloseReload(partials.ModalCloseReloadInput{
				Trigger: partials.ModalCloseReloadCategories,
			}))
		}).Name(CategoryCreateRoute)

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
				DisplayName string `form:"display_name"`
				IsShelved   bool   `form:"is_shelved"`
			}
			if err := c.BodyParser(&body); err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid body")
			}

			if err := deps.CategoryUpdate.Handle(c.Context(), user.UserID, command.UpdateUserCategoryCmd{
				ID:          uint(id64),
				DisplayName: body.DisplayName,
				IsShelved:   body.IsShelved,
			}); err != nil {
				return httpError(err)
			}

			return middleware.Render(c, partials.ModalCloseReload(partials.ModalCloseReloadInput{
				Trigger: partials.ModalCloseReloadCategories,
			}))
		}).Name(CategoryUpdateRoute)

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

			if err := deps.CategoryDelete.Handle(c.Context(), user.UserID, uint(id64)); err != nil {
				return httpError(err)
			}

			return middleware.Render(c, partials.ModalCloseReload(partials.ModalCloseReloadInput{
				Trigger: partials.ModalCloseReloadCategories,
			}))
		}).Name(CategoryDeleteRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/shelved", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			shelved, err := deps.GetUserShelvedCategories.Handle(c.Context(), user.UserID)
			if err != nil {
				return err
			}

			inputs := lo.Map(shelved, func(category model.Category, _ int) partials.CategoriesShelvedInput {
				return partials.CategoriesShelvedInput{
					ID:          category.ID,
					DisplayName: category.DisplayName,
					Bookmarks: lo.Map(category.Bookmarks, func(bookmark model.Bookmark, _ int) partials.CategoriesShelvedInputBookmark {
						return partials.CategoriesShelvedInputBookmark{
							ID:          bookmark.ID,
							IconType:    bookmark.Icon.Type(),
							Icon:        bookmark.Icon.Name(),
							DisplayName: bookmark.DisplayName,
							Url:         bookmark.Url.String(),
							Domain:      bookmark.Url.Host(),
						}
					}),
				}
			})
			return middleware.Render(c, partials.CategoriesShelved(inputs))
		}).Name(CategoriesShelvedRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/shelved/edit", func(c *fiber.Ctx) error {
			user, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			shelved, err := deps.GetUserShelvedCategories.Handle(c.Context(), user.UserID)
			if err != nil {
				return err
			}

			inputs := lo.Map(shelved, func(category model.Category, _ int) partials.CategoriesShelvedEditInput {
				return partials.CategoriesShelvedEditInput{
					ID:          category.ID,
					DisplayName: category.DisplayName,
					Bookmarks: lo.Map(category.Bookmarks, func(bookmark model.Bookmark, _ int) partials.CategoriesShelvedEditInputBookmark {
						return partials.CategoriesShelvedEditInputBookmark{
							ID:          bookmark.ID,
							IconType:    bookmark.Icon.Type(),
							Icon:        bookmark.Icon.Name(),
							DisplayName: bookmark.DisplayName,
							Domain:      bookmark.Url.Host(),
						}
					}),
				}
			})
			return middleware.Render(c, partials.CategoriesShelvedEdit(inputs))
		}).Name(CategoriesShelvedEditRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/modal/create", func(c *fiber.Ctx) error {
			return middleware.Render(c, partials.CategoriesCreateModal())
		}).Name(CategoriesModalCreateRoute)

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

			category, err := deps.GetUserCategory.Handle(c.Context(), user.UserID, uint(id64))
			if err != nil {
				return httpError(err)
			}

			return middleware.Render(c, partials.CategoriesEditModal(partials.CategoriesEditModalInput{
				ID:          category.ID,
				DisplayName: category.DisplayName,
				IsShelved:   category.IsShelved,
			}))
		}).Name(CategoriesModalEditRoute)

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

			category, err := deps.GetUserCategory.Handle(c.Context(), user.UserID, uint(id64))
			if err != nil {
				return httpError(err)
			}

			return middleware.Render(c, partials.CategoriesDeleteModal(partials.CategoriesDeleteModalInput{
				ID:          category.ID,
				DisplayName: category.DisplayName,
			}))
		}).Name(CategoriesModalDeleteRoute)

	router.
		Use(middleware.HtmxOnly).
		Get("/modal/shelved/:isShelved", func(c *fiber.Ctx) error {
			_, authorized := middleware.GetCurrentUser(c)
			if !authorized {
				return redirectToLogin(c)
			}

			isShelved, err := strconv.ParseBool(c.Params("isShelved"))
			if err != nil {
				return fiber.NewError(fiber.StatusBadRequest, "invalid isShelved")
			}

			return middleware.Render(c, partials.CategoriesShelvedModalButton(partials.CategoriesShelvedModalButtonInput{
				IsShelved: isShelved,
			}))
		}).Name(CategoriesModalShelvedRoute)
}
