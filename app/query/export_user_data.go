package query

import (
	"context"
	"errors"
	"strconv"
	"strings"
	"time"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainrepo "github.com/oechsler-it/dash/domain/repo"
	"github.com/oechsler-it/dash/app/transfer"
)

// UserDataExporter handles the export-user-data query.
type UserDataExporter interface {
	Handle(ctx context.Context, userID string, username string, isAdmin bool) (*transfer.UserDataExport, error)
}

type ExportUserData struct {
	DashboardRepo   domainrepo.DashboardRepository
	CategoryRepo    domainrepo.CategoryRepository
	BookmarkRepo    domainrepo.BookmarkRepository
	ThemeRepo       domainrepo.ThemeRepository
	SettingRepo     domainrepo.SettingRepository
	ApplicationRepo domainrepo.ApplicationRepository
}

func NewExportUserData(
	dashboardRepo domainrepo.DashboardRepository,
	categoryRepo domainrepo.CategoryRepository,
	bookmarkRepo domainrepo.BookmarkRepository,
	themeRepo domainrepo.ThemeRepository,
	settingRepo domainrepo.SettingRepository,
	applicationRepo domainrepo.ApplicationRepository,
) *ExportUserData {
	return &ExportUserData{
		DashboardRepo:   dashboardRepo,
		CategoryRepo:    categoryRepo,
		BookmarkRepo:    bookmarkRepo,
		ThemeRepo:       themeRepo,
		SettingRepo:     settingRepo,
		ApplicationRepo: applicationRepo,
	}
}

func (h *ExportUserData) Handle(ctx context.Context, userID string, username string, isAdmin bool) (*transfer.UserDataExport, error) {
	export := &transfer.UserDataExport{
		Version:    1,
		ExportedAt: time.Now().UTC(),
		Username:   username,
		Categories: []transfer.CategoryExport{},
		Themes:     []transfer.ThemeExport{},
	}

	// Settings
	var activeThemeID uint
	setting, err := h.SettingRepo.GetByUserID(ctx, userID)
	if err != nil {
		var nfe *domainerrors.NotFoundError
		if !errors.As(err, &nfe) {
			return nil, domainerrors.Internal("export user data: get settings", err)
		}
	} else {
		activeThemeID = setting.ThemeID
		export.Settings = transfer.SettingsExport{
			Language: setting.Language,
			Timezone: setting.Timezone,
		}
	}

	// Themes (user-created only, i.e. Deletable = true)
	themes, err := h.ThemeRepo.ListByUser(ctx, userID)
	if err != nil {
		return nil, domainerrors.Internal("export user data: list themes", err)
	}
	// Resolve active theme name for settings export
	for _, t := range themes {
		if t.ID == activeThemeID {
			export.Settings.ThemeName = t.DisplayName
			break
		}
	}
	for _, t := range themes {
		if !t.Deletable {
			continue
		}
		export.Themes = append(export.Themes, transfer.ThemeExport{
			Hash:      transfer.ContentHash(t.DisplayName, t.Primary, t.Secondary, t.Tertiary),
			Name:      t.DisplayName,
			Primary:   t.Primary,
			Secondary: t.Secondary,
			Tertiary:  t.Tertiary,
		})
	}

	// Categories + Bookmarks
	dashboard, err := h.DashboardRepo.GetByUserID(ctx, userID)
	if err != nil {
		var nfe *domainerrors.NotFoundError
		if !errors.As(err, &nfe) {
			return nil, domainerrors.Internal("export user data: get dashboard", err)
		}
		// No dashboard yet → empty categories
		return export, nil
	}

	categories, err := h.CategoryRepo.ListByDashboardID(ctx, dashboard.ID)
	if err != nil {
		return nil, domainerrors.Internal("export user data: list categories", err)
	}

	categoryIDs := make([]uint, len(categories))
	for i, c := range categories {
		categoryIDs[i] = c.ID
	}

	bookmarks, err := h.BookmarkRepo.ListByCategoryIDs(ctx, categoryIDs)
	if err != nil {
		return nil, domainerrors.Internal("export user data: list bookmarks", err)
	}

	bookmarksByCategory := make(map[uint][]domainrepo.BookmarkRecord)
	for _, b := range bookmarks {
		bookmarksByCategory[b.CategoryID] = append(bookmarksByCategory[b.CategoryID], b)
	}

	for _, c := range categories {
		catExport := transfer.CategoryExport{
			Hash:        transfer.ContentHash(c.DisplayName, strconv.FormatBool(c.IsShelved)),
			DisplayName: c.DisplayName,
			IsShelved:   c.IsShelved,
			Bookmarks:   []transfer.BookmarkExport{},
		}
		for _, b := range bookmarksByCategory[c.ID] {
			catExport.Bookmarks = append(catExport.Bookmarks, transfer.BookmarkExport{
				Hash:        transfer.ContentHash(b.Icon, b.DisplayName, b.Url),
				Icon:        b.Icon,
				DisplayName: b.DisplayName,
				URL:         b.Url,
			})
		}
		export.Categories = append(export.Categories, catExport)
	}

	// Applications (admin only)
	if isAdmin {
		apps, err := h.ApplicationRepo.List(ctx)
		if err != nil {
			return nil, domainerrors.Internal("export user data: list applications", err)
		}
		export.Applications = make([]transfer.ApplicationExport, 0, len(apps))
		for _, a := range apps {
			groups := a.VisibleToGroups
			if groups == nil {
				groups = []string{}
			}
			export.Applications = append(export.Applications, transfer.ApplicationExport{
				Hash:            transfer.ContentHash(a.Icon, a.DisplayName, a.Url, strings.Join(groups, ",")),
				Icon:            a.Icon,
				DisplayName:     a.DisplayName,
				URL:             a.Url,
				VisibleToGroups: groups,
			})
		}
	}

	return export, nil
}
