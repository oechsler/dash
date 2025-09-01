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
	GetDefault(ctx context.Context, userID string) (*model.Theme, error)
	EnsureDefault(ctx context.Context, userID string) (*model.Theme, error)
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
	if err := r.db.WithContext(ctx).Where("user_id = ?", userID).Find(&list).Error; err != nil {
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

func (r *GormThemeRepo) GetDefault(ctx context.Context, userID string) (*model.Theme, error) {
	// For now first theme is default. Later can add flag.
	var t model.Theme
	if err := r.db.WithContext(ctx).Where("user_id = ?", userID).First(&t).Error; err != nil {
		return nil, err
	}
	return &t, nil
}

func (r *GormThemeRepo) EnsureDefault(ctx context.Context, userID string) (*model.Theme, error) {
	var count int64
	if err := r.db.WithContext(ctx).Model(&model.Theme{}).Where("user_id = ?", userID).Count(&count).Error; err != nil {
		return nil, err
	}
	if count == 0 {
		def := &model.Theme{UserId: userID, Name: "Catppuccin Mocha", Primary: "#1e1e2e", Secondary: "#cdd6f4", Tertiary: "#cba6f7", Deletable: false}
		if err := r.Create(ctx, def); err != nil {
			return nil, err
		}
		return def, nil
	}
	// Ensure there is at least one non-deletable theme for this user. This also fixes
	// older rows that might have been created with Deletable=true due to DB defaults.
	var nonDel model.Theme
	if err := r.db.WithContext(ctx).Where("user_id = ? AND deletable = ?", userID, false).First(&nonDel).Error; err == nil {
		return &nonDel, nil
	}
	// No non-deletable theme found: pick the oldest theme and mark it as non-deletable.
	var first model.Theme
	if err := r.db.WithContext(ctx).Where("user_id = ?", userID).Order("id ASC").First(&first).Error; err != nil {
		return nil, err
	}
	if err := r.db.WithContext(ctx).Model(&model.Theme{}).Where("id = ?", first.ID).Update("deletable", false).Error; err != nil {
		return nil, err
	}
	first.Deletable = false
	return &first, nil
}
