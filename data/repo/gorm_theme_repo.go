package repo

import (
	"context"
	"dash/data/model"

	"gorm.io/gorm"
)

type ThemeRepo interface {
	Create(ctx context.Context, t *model.Theme) error
	Delete(ctx context.Context, userID string, id uint) error
	ListByUser(ctx context.Context, userID string) ([]model.Theme, error)
	GetByID(ctx context.Context, userID string, id uint) (*model.Theme, error)
}

type GormThemeRepo struct{ db *gorm.DB }

func NewGormThemeRepo(db *gorm.DB) (*GormThemeRepo, error) {
	if err := db.AutoMigrate(&model.Theme{}); err != nil {
		return nil, err
	}
	return &GormThemeRepo{db: db}, nil
}

func (r *GormThemeRepo) Create(ctx context.Context, t *model.Theme) error {
	// Use Select("*") to ensure zero-value fields (like Deletable=false) are persisted
	// even when the model defines a DB default. GORM omits zero-values on Create when
	// a default tag is present to leverage DB defaults; this forces explicit persistence.
	return r.db.WithContext(ctx).Select("*").Create(t).Error
}

func (r *GormThemeRepo) Delete(ctx context.Context, userID string, id uint) error {
	// Only delete themes that are marked as deletable
	return r.db.WithContext(ctx).Where("user_id = ? AND id = ? AND deletable = ?", userID, id, true).Delete(&model.Theme{}).Error
}

func (r *GormThemeRepo) ListByUser(ctx context.Context, userID string) ([]model.Theme, error) {
	var list []model.Theme
	if err := r.db.WithContext(ctx).Where("user_id = ?", userID).Order("display_name COLLATE NOCASE ASC, id ASC").Find(&list).Error; err != nil {
		return nil, err
	}
	return list, nil
}

func (r *GormThemeRepo) GetByID(ctx context.Context, userID string, id uint) (*model.Theme, error) {
	var t model.Theme
	if err := r.db.WithContext(ctx).Where("user_id = ? AND id = ?", userID, id).First(&t).Error; err != nil {
		return nil, err
	}
	return &t, nil
}
