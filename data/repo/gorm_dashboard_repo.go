package repo

import (
	"context"
	"dash/data/model"
	"errors"

	"gorm.io/gorm"
)

type DashboardRepo interface {
	Upsert(ctx context.Context, dashboard *model.Dashboard) error
	Get(ctx context.Context, id uint) (*model.Dashboard, error)
	GetByUserId(ctx context.Context, userID string) (*model.Dashboard, error)
	Delete(ctx context.Context, id uint) error
}

type GormDashboardRepo struct {
	db *gorm.DB
}

func NewGormDashboardRepo(db *gorm.DB) (*GormDashboardRepo, error) {
	if err := db.AutoMigrate(&model.Dashboard{}); err != nil {
		return nil, err
	}
	return &GormDashboardRepo{db: db}, nil
}

func (r *GormDashboardRepo) Upsert(ctx context.Context, dashboard *model.Dashboard) error {
	return r.db.WithContext(ctx).Save(dashboard).Error
}

func (r *GormDashboardRepo) Get(ctx context.Context, id uint) (*model.Dashboard, error) {
	var dashboard model.Dashboard
	err := r.db.WithContext(ctx).First(&dashboard, id).Error
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, nil
		}
		return nil, err
	}
	return &dashboard, nil
}

func (r *GormDashboardRepo) GetByUserId(ctx context.Context, userID string) (*model.Dashboard, error) {
	var dashboard model.Dashboard
	err := r.db.WithContext(ctx).Where("user_id = ?", userID).First(&dashboard).Error
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, nil
		}
		return nil, err
	}
	return &dashboard, nil
}

func (r *GormDashboardRepo) Delete(ctx context.Context, id uint) error {
	return r.db.WithContext(ctx).Delete(&model.Dashboard{}, id).Error
}
