package repo

import (
	"context"
	"errors"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	"git.at.oechsler.it/samuel/dash/v2/infra/persistence/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"

	"gorm.io/gorm"
)

var _ domainrepo.ThemeRepository = (*GormThemeRepo)(nil)

type GormThemeRepo struct{ db *gorm.DB }

func NewGormThemeRepo(db *gorm.DB) (*GormThemeRepo, error) {
	if err := db.AutoMigrate(&model.Theme{}); err != nil {
		return nil, err
	}
	return &GormThemeRepo{db: db}, nil
}

func (r *GormThemeRepo) Create(ctx context.Context, record *domainrepo.ThemeRecord) error {
	m := &model.Theme{
		UserId:      record.UserID,
		DisplayName: record.DisplayName,
		Primary:     record.Primary,
		Secondary:   record.Secondary,
		Tertiary:    record.Tertiary,
		Deletable:   record.Deletable,
	}
	// Use Select("*") to ensure zero-value fields (like Deletable=false) are persisted
	// even when the model defines a DB default. GORM omits zero-values on Create when
	// a default tag is present to leverage DB defaults; this forces explicit persistence.
	if err := r.db.WithContext(ctx).Select("*").Create(m).Error; err != nil {
		return err
	}
	record.ID = m.ID
	return nil
}

func (r *GormThemeRepo) DeleteAllByUser(ctx context.Context, userID string) error {
	return r.db.WithContext(ctx).Where("user_id = ?", userID).Delete(&model.Theme{}).Error
}

func (r *GormThemeRepo) Delete(ctx context.Context, userID string, id uint) error {
	// Only delete themes that are marked as deletable
	return r.db.WithContext(ctx).Where("user_id = ? AND id = ? AND deletable = ?", userID, id, true).Delete(&model.Theme{}).Error
}

func (r *GormThemeRepo) ListByUser(ctx context.Context, userID string) ([]domainrepo.ThemeRecord, error) {
	var list []model.Theme
	if err := r.db.WithContext(ctx).Where("user_id = ?", userID).Order("LOWER(display_name) ASC, id ASC").Find(&list).Error; err != nil {
		return nil, err
	}
	records := make([]domainrepo.ThemeRecord, len(list))
	for i, t := range list {
		records[i] = domainrepo.ThemeRecord{
			ID:          t.ID,
			UserID:      t.UserId,
			DisplayName: t.DisplayName,
			Primary:     t.Primary,
			Secondary:   t.Secondary,
			Tertiary:    t.Tertiary,
			Deletable:   t.Deletable,
		}
	}
	return records, nil
}

// GetByID returns the theme for the given user and id, or a NotFoundError if not found.
func (r *GormThemeRepo) GetByID(ctx context.Context, userID string, id uint) (*domainrepo.ThemeRecord, error) {
	var t model.Theme
	if err := r.db.WithContext(ctx).Where("user_id = ? AND id = ?", userID, id).First(&t).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, domainerrors.NotFound(domainerrors.EntityTheme)
		}
		return nil, err
	}
	return &domainrepo.ThemeRecord{
		ID:          t.ID,
		UserID:      t.UserId,
		DisplayName: t.DisplayName,
		Primary:     t.Primary,
		Secondary:   t.Secondary,
		Tertiary:    t.Tertiary,
		Deletable:   t.Deletable,
	}, nil
}
