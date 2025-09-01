package repo

import (
	"context"
	"dash/data/model"
	"errors"

	"gorm.io/gorm"
)

type CategoryRepo interface {
	Upsert(ctx context.Context, category *model.Category) error
	Get(ctx context.Context, id uint) (*model.Category, error)
	ListByDashboardID(ctx context.Context, dashboardID uint) ([]model.Category, error)
	Delete(ctx context.Context, id uint) error
}

type GormCategoryRepo struct {
	db *gorm.DB
}

func NewGormCategoryRepo(db *gorm.DB) (*GormCategoryRepo, error) {
	if err := db.AutoMigrate(&model.Category{}); err != nil {
		return nil, err
	}
	return &GormCategoryRepo{db: db}, nil
}

func (r *GormCategoryRepo) Upsert(ctx context.Context, category *model.Category) error {
	return r.db.WithContext(ctx).Save(category).Error
}

func (r *GormCategoryRepo) Get(ctx context.Context, id uint) (*model.Category, error) {
	var c model.Category
	if err := r.db.WithContext(ctx).First(&c, id).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, nil
		}
		return nil, err
	}
	return &c, nil
}

func (r *GormCategoryRepo) ListByDashboardID(ctx context.Context, dashboardID uint) ([]model.Category, error) {
	var list []model.Category
	if err := r.db.WithContext(ctx).
		Where("dashboard_id = ?", dashboardID).
		Find(&list).Error; err != nil {
		return nil, err
	}
	return list, nil
}

func (r *GormCategoryRepo) Delete(ctx context.Context, id uint) error {
	return r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		if err := tx.Where("category_id = ?", id).Delete(&model.Bookmark{}).Error; err != nil {
			return err
		}
		if err := tx.Delete(&model.Category{}, id).Error; err != nil {
			return err
		}
		return nil
	})
}
