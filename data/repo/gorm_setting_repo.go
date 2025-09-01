package repo

import (
	"context"
	"dash/data/model"
	"errors"

	"gorm.io/gorm"
)

type SettingRepo interface {
	Upsert(ctx context.Context, setting *model.Setting) error
	GetByUserId(ctx context.Context, userID string) (*model.Setting, error)
}

type GormSettingRepo struct {
	db *gorm.DB
}

func NewGormSettingRepo(db *gorm.DB) (*GormSettingRepo, error) {
	if err := db.AutoMigrate(&model.Setting{}); err != nil {
		return nil, err
	}
	return &GormSettingRepo{db: db}, nil
}

func (r *GormSettingRepo) Upsert(ctx context.Context, setting *model.Setting) error {
	return r.db.WithContext(ctx).Save(setting).Error
}

func (r *GormSettingRepo) GetByUserId(ctx context.Context, userID string) (*model.Setting, error) {
	var s model.Setting
	err := r.db.WithContext(ctx).Where("user_id = ?", userID).First(&s).Error
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, nil
		}
		return nil, err
	}
	return &s, nil
}
