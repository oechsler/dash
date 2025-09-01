package repo

import (
	"context"
	"dash/data/model"
	"errors"

	"gorm.io/gorm"
)

type ApplicationRepo interface {
	Upsert(ctx context.Context, application *model.Application) error
	Get(ctx context.Context, id uint) (*model.Application, error)
	List(ctx context.Context) ([]model.Application, error)
	Delete(ctx context.Context, id uint) error
}

type GormApplicationRepo struct {
	db *gorm.DB
}

func NewGormApplicationRepo(db *gorm.DB) (*GormApplicationRepo, error) {
	if err := db.AutoMigrate(&model.Application{}); err != nil {
		return nil, err
	}
	return &GormApplicationRepo{db: db}, nil
}

func (r *GormApplicationRepo) Upsert(ctx context.Context, application *model.Application) error {
	return r.db.WithContext(ctx).Save(application).Error
}

func (r *GormApplicationRepo) Get(ctx context.Context, id uint) (*model.Application, error) {
	var app model.Application
	if err := r.db.WithContext(ctx).First(&app, id).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, nil
		}
		return nil, err
	}
	return &app, nil
}

func (r *GormApplicationRepo) List(ctx context.Context) ([]model.Application, error) {
	var apps []model.Application
	if err := r.db.WithContext(ctx).Find(&apps).Error; err != nil {
		return nil, err
	}
	return apps, nil
}

func (r *GormApplicationRepo) Delete(ctx context.Context, id uint) error {
	return r.db.WithContext(ctx).Delete(&model.Application{}, id).Error
}
