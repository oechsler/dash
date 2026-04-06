package repo

import (
	"context"
	"errors"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	"git.at.oechsler.it/samuel/dash/v2/infra/persistence/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"

	"gorm.io/gorm"
)

var _ domainrepo.SettingRepository = (*GormSettingRepo)(nil)

type GormSettingRepo struct {
	db *gorm.DB
}

func NewGormSettingRepo(db *gorm.DB) (*GormSettingRepo, error) {
	if err := db.AutoMigrate(&model.Setting{}); err != nil {
		return nil, err
	}
	return &GormSettingRepo{db: db}, nil
}

func (r *GormSettingRepo) Upsert(ctx context.Context, record *domainrepo.SettingRecord) error {
	m := &model.Setting{
		UserId:   record.UserID,
		ThemeID:  record.ThemeID,
		Language: record.Language,
		Timezone: record.Timezone,
	}
	if record.ID != 0 {
		m.ID = record.ID
	}
	return r.db.WithContext(ctx).Save(m).Error
}

func (r *GormSettingRepo) DeleteByUserID(ctx context.Context, userID string) error {
	return r.db.WithContext(ctx).Where("user_id = ?", userID).Delete(&model.Setting{}).Error
}

func (r *GormSettingRepo) GetByUserID(ctx context.Context, userID string) (*domainrepo.SettingRecord, error) {
	var s model.Setting
	err := r.db.WithContext(ctx).Where("user_id = ?", userID).First(&s).Error
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, domainerrors.NotFound(domainerrors.EntitySetting)
		}
		return nil, err
	}
	return &domainrepo.SettingRecord{ID: s.ID, UserID: s.UserId, ThemeID: s.ThemeID, Language: s.Language, Timezone: s.Timezone}, nil
}
