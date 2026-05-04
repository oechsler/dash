package repo

import (
	"context"
	"errors"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	"git.at.oechsler.it/samuel/dash/v2/infra/persistence/model"

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
	// Drop the FK constraint and NOT NULL on theme_id introduced before the
	// synthetic-default migration. theme_id = NULL now means "use synthetic default".
	noPS := db.Session(&gorm.Session{PrepareStmt: false})
	if err := noPS.Exec(`
		DO $$
		BEGIN
			IF EXISTS (
				SELECT 1 FROM pg_constraint WHERE conname = 'fk_settings_theme'
			) THEN
				ALTER TABLE settings DROP CONSTRAINT fk_settings_theme;
			END IF;
			-- Make nullable in case AutoMigrate didn't manage it yet.
			ALTER TABLE settings ALTER COLUMN theme_id DROP NOT NULL;
		EXCEPTION WHEN others THEN
			NULL; -- column is already nullable, ignore
		END
		$$
	`).Error; err != nil {
		return nil, err
	}
	return &GormSettingRepo{db: db}, nil
}

func (r *GormSettingRepo) Upsert(ctx context.Context, record *domainrepo.SettingRecord) error {
	var themeID *uint
	if !domainmodel.IsDefaultThemeID(record.ThemeID) {
		themeID = &record.ThemeID
	}
	m := &model.Setting{
		UserID:   record.UserID,
		ThemeID:  themeID,
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
	themeID := domainmodel.DefaultTheme().ID
	if s.ThemeID != nil {
		themeID = *s.ThemeID
	}
	return &domainrepo.SettingRecord{ID: s.ID, UserID: s.UserID, ThemeID: themeID, Language: s.Language, Timezone: s.Timezone}, nil
}
