package handler

import (
	"sort"
	"strconv"

	"git.at.oechsler.it/samuel/dash/v2/app/command"
	"git.at.oechsler.it/samuel/dash/v2/app/query"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/middleware"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/templ/components"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/templ/partials"
	"git.at.oechsler.it/samuel/dash/v2/infra/oidc"

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
	SessionStore             *oidc.SessionStore
	App                      *fiber.App
	GetUserCategory          query.UserCategoryGetter
	GetUserBookmark          query.UserBookmarkGetter
	GetUserCategories        query.UserCategoriesGetter
	GetUserShelvedCategories query.UserShelvedCategoriesGetter
	BookmarkCreate           command.UserBookmarkCreator
	BookmarkUpdate           command.UserBookmarkUpdater
	BookmarkDelete           command.UserBookmarkDeleter
	GetAvailableIconTypes    query.AvailableIconTypesGetter
}

func Bookmark(deps BookmarkDeps) {
	router := deps.App.
		Group("/bookmarks").
		Use(middleware.LoadUserFromSession(deps.SessionStore))

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

			if err := deps.BookmarkCreate.Handle(c.Context(), user.UserID, command.CreateUserBookmarkCmd{
				Icon:        body.IconType + ":" + body.IconName,
				DisplayName: body.DisplayName,
				Url:         body.Url,
				CategoryID:  body.CategoryID,
			}); err != nil {
				return httpError(err)
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

			if err := deps.BookmarkUpdate.Handle(c.Context(), user.UserID, command.UpdateUserBookmarkCmd{
				ID:          uint(id64),
				Icon:        body.IconType + ":" + body.IconName,
				DisplayName: body.DisplayName,
				Url:         body.Url,
				CategoryID:  body.CategoryID,
			}); err != nil {
				return httpError(err)
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

			if err := deps.BookmarkDelete.Handle(c.Context(), user.UserID, uint(id64)); err != nil {
				return httpError(err)
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

			category, err := deps.GetUserCategory.Handle(c.Context(), user.UserID, uint(id64))
			if err != nil {
				return httpError(err)
			}

			return middleware.Render(c, partials.BookmarksCreateModal(partials.BookmarksCreateModalInput{
				CategoryID:          category.ID,
				CategoryDisplayName: category.DisplayName,
				IconTypes: func() components.ModalUpserInputIconTypes {
					list, _ := deps.GetAvailableIconTypes.Handle(c.Context())
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

			bookmark, err := deps.GetUserBookmark.Handle(c.Context(), user.UserID, uint(id64))
			if err != nil {
				return httpError(err)
			}

			categories, _ := deps.GetUserCategories.Handle(c.Context(), user.UserID)
			shelvedCategories, _ := deps.GetUserShelvedCategories.Handle(c.Context(), user.UserID)
			allCategories := append(categories, shelvedCategories...)
			sort.Slice(allCategories, func(i, j int) bool {
				return allCategories[i].DisplayName < allCategories[j].DisplayName
			})
			return middleware.Render(c, partials.BookmarksEditModal(partials.BookmarksEditModalInput{
				ID: bookmark.ID,
				IconTypes: func() components.ModalUpserInputIconTypes {
					list, _ := deps.GetAvailableIconTypes.Handle(c.Context())
					return components.ModalUpserInputIconTypes(list)
				}(),
				Icon: components.ModalUpsertInputIcon{
					Type: bookmark.Icon.Type(),
					Name: bookmark.Icon.Name(),
				},
				DisplayName: bookmark.DisplayName,
				Url:         bookmark.Url.String(),
				CategoryID:  bookmark.CategoryID,
				Categories: func() []partials.BookmarksEditModalInputCategory {
					res := make([]partials.BookmarksEditModalInputCategory, 0, len(allCategories))
					for _, cat := range allCategories {
						res = append(res, partials.BookmarksEditModalInputCategory{ID: cat.ID, DisplayName: cat.DisplayName})
					}
					return res
				}(),
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

			bookmark, err := deps.GetUserBookmark.Handle(c.Context(), user.UserID, uint(id64))
			if err != nil {
				return httpError(err)
			}

			return middleware.Render(c, partials.BookmarksDeleteModal(partials.BookmarksDeleteModalInput{
				ID:          bookmark.ID,
				DisplayName: bookmark.DisplayName,
			}))
		}).Name(BookmarkModalDeleteRoute)
}
