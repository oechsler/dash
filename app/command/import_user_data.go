package command

import (
	"context"
	"errors"
	"strconv"
	"strings"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainrepo "github.com/oechsler-it/dash/domain/repo"
	"github.com/oechsler-it/dash/app/transfer"
)

// UserDataImporter handles the import-user-data command.
type UserDataImporter interface {
	Handle(ctx context.Context, userID string, isAdmin bool, in *transfer.UserDataExport) error
}

type ImportUserData struct {
	DashboardRepo      domainrepo.DashboardRepository
	CategoryRepo       domainrepo.CategoryRepository
	BookmarkRepo       domainrepo.BookmarkRepository
	ThemeRepo          domainrepo.ThemeRepository
	SettingRepo        domainrepo.SettingRepository
	ApplicationRepo    domainrepo.ApplicationRepository
	EnsureDefaultTheme *EnsureDefaultTheme
}

func NewImportUserData(
	dashboardRepo domainrepo.DashboardRepository,
	categoryRepo domainrepo.CategoryRepository,
	bookmarkRepo domainrepo.BookmarkRepository,
	themeRepo domainrepo.ThemeRepository,
	settingRepo domainrepo.SettingRepository,
	applicationRepo domainrepo.ApplicationRepository,
	ensureDefaultTheme *EnsureDefaultTheme,
) *ImportUserData {
	return &ImportUserData{
		DashboardRepo:      dashboardRepo,
		CategoryRepo:       categoryRepo,
		BookmarkRepo:       bookmarkRepo,
		ThemeRepo:          themeRepo,
		SettingRepo:        settingRepo,
		ApplicationRepo:    applicationRepo,
		EnsureDefaultTheme: ensureDefaultTheme,
	}
}

func (h *ImportUserData) Handle(ctx context.Context, userID string, isAdmin bool, in *transfer.UserDataExport) error {
	// --- Load existing hashes for deduplication ---

	existingThemeHashes := map[string]struct{}{}
	existingThemes, err := h.ThemeRepo.ListByUser(ctx, userID)
	if err != nil {
		return domainerrors.Internal("import user data: list existing themes", err)
	}
	for _, t := range existingThemes {
		existingThemeHashes[transfer.ContentHash(t.DisplayName, t.Primary, t.Secondary, t.Tertiary)] = struct{}{}
	}

	existingCategoryHashes := map[string]struct{}{}
	var dashboardID uint
	dashboard, err := h.DashboardRepo.GetByUserID(ctx, userID)
	if err != nil {
		var nfe *domainerrors.NotFoundError
		if !errors.As(err, &nfe) {
			return domainerrors.Internal("import user data: get dashboard", err)
		}
	} else {
		dashboardID = dashboard.ID
		categories, err := h.CategoryRepo.ListByDashboardID(ctx, dashboard.ID)
		if err != nil {
			return domainerrors.Internal("import user data: list existing categories", err)
		}
		for _, c := range categories {
			existingCategoryHashes[transfer.ContentHash(c.DisplayName, strconv.FormatBool(c.IsShelved))] = struct{}{}
		}
	}

	existingAppHashes := map[string]struct{}{}
	if isAdmin {
		apps, err := h.ApplicationRepo.List(ctx)
		if err != nil {
			return domainerrors.Internal("import user data: list existing applications", err)
		}
		for _, a := range apps {
			groups := a.VisibleToGroups
			if groups == nil {
				groups = []string{}
			}
			existingAppHashes[transfer.ContentHash(a.Icon, a.DisplayName, a.Url, strings.Join(groups, ","))] = struct{}{}
		}
	}

	// --- Ensure default theme ---
	defaultTheme, err := h.EnsureDefaultTheme.Handle(ctx, userID)
	if err != nil {
		return err
	}

	// --- Import themes ---
	nameToThemeID := map[string]uint{}
	// Seed with existing themes so settings resolution works for pre-existing themes
	for _, t := range existingThemes {
		nameToThemeID[t.DisplayName] = t.ID
	}

	for _, t := range in.Themes {
		if _, exists := existingThemeHashes[t.Hash]; exists {
			continue
		}
		rec := &domainrepo.ThemeRecord{
			UserID:      userID,
			DisplayName: t.Name,
			Primary:     t.Primary,
			Secondary:   t.Secondary,
			Tertiary:    t.Tertiary,
			Deletable:   true,
		}
		if err := h.ThemeRepo.Create(ctx, rec); err != nil {
			return domainerrors.Internal("import user data: create theme", err)
		}
		nameToThemeID[t.Name] = rec.ID
		existingThemeHashes[t.Hash] = struct{}{}
	}

	// --- Ensure dashboard exists ---
	if dashboardID == 0 {
		rec := &domainrepo.DashboardRecord{UserID: userID}
		if err := h.DashboardRepo.Upsert(ctx, rec); err != nil {
			return domainerrors.Internal("import user data: upsert dashboard", err)
		}
		d, err := h.DashboardRepo.GetByUserID(ctx, userID)
		if err != nil {
			return domainerrors.Internal("import user data: get dashboard after upsert", err)
		}
		dashboardID = d.ID
	}

	// --- Import categories and bookmarks ---
	for _, cat := range in.Categories {
		catHash := transfer.ContentHash(cat.DisplayName, strconv.FormatBool(cat.IsShelved))
		var catID uint
		if _, exists := existingCategoryHashes[catHash]; !exists {
			rec := &domainrepo.CategoryRecord{
				DashboardID: dashboardID,
				DisplayName: cat.DisplayName,
				IsShelved:   cat.IsShelved,
			}
			if err := h.CategoryRepo.Upsert(ctx, rec); err != nil {
				return domainerrors.Internal("import user data: upsert category", err)
			}
			catID = rec.ID
			existingCategoryHashes[catHash] = struct{}{}
		} else {
			// Find existing category ID by matching display name + shelved state
			cats, err := h.CategoryRepo.ListByDashboardID(ctx, dashboardID)
			if err != nil {
				return domainerrors.Internal("import user data: list categories for bookmark lookup", err)
			}
			for _, c := range cats {
				if c.DisplayName == cat.DisplayName && c.IsShelved == cat.IsShelved {
					catID = c.ID
					break
				}
			}
		}

		if catID == 0 {
			continue
		}

		// Load existing bookmarks for this category to deduplicate
		existingBookmarkHashes := map[string]struct{}{}
		existingBms, err := h.BookmarkRepo.ListByCategoryIDs(ctx, []uint{catID})
		if err != nil {
			return domainerrors.Internal("import user data: list existing bookmarks for category", err)
		}
		for _, b := range existingBms {
			existingBookmarkHashes[transfer.ContentHash(b.Icon, b.DisplayName, b.Url)] = struct{}{}
		}

		for _, bm := range cat.Bookmarks {
			if _, exists := existingBookmarkHashes[bm.Hash]; exists {
				continue
			}
			rec := &domainrepo.BookmarkRecord{
				CategoryID:  catID,
				Icon:        bm.Icon,
				DisplayName: bm.DisplayName,
				Url:         bm.URL,
			}
			if err := h.BookmarkRepo.Upsert(ctx, rec); err != nil {
				return domainerrors.Internal("import user data: upsert bookmark", err)
			}
			existingBookmarkHashes[bm.Hash] = struct{}{}
		}
	}

	// --- Import applications (admin only) ---
	if isAdmin {
		for _, a := range in.Applications {
			if _, exists := existingAppHashes[a.Hash]; exists {
				continue
			}
			groups := a.VisibleToGroups
			if groups == nil {
				groups = []string{}
			}
			rec := &domainrepo.ApplicationRecord{
				Icon:            a.Icon,
				DisplayName:     a.DisplayName,
				Url:             a.URL,
				VisibleToGroups: groups,
			}
			if err := h.ApplicationRepo.Upsert(ctx, rec); err != nil {
				return domainerrors.Internal("import user data: upsert application", err)
			}
			existingAppHashes[a.Hash] = struct{}{}
		}
	}

	// --- Upsert settings ---
	themeID := defaultTheme.ID
	if in.Settings.ThemeName != "" {
		if id, ok := nameToThemeID[in.Settings.ThemeName]; ok {
			themeID = id
		}
	}

	existing, err := h.SettingRepo.GetByUserID(ctx, userID)
	if err != nil {
		var nfe *domainerrors.NotFoundError
		if !errors.As(err, &nfe) {
			return domainerrors.Internal("import user data: get settings", err)
		}
		existing = &domainrepo.SettingRecord{UserID: userID}
	}
	existing.ThemeID = themeID
	existing.Language = in.Settings.Language
	existing.Timezone = in.Settings.Timezone
	if err := h.SettingRepo.Upsert(ctx, existing); err != nil {
		return domainerrors.Internal("import user data: upsert settings", err)
	}

	return nil
}
