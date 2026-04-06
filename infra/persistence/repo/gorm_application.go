package repo

import (
	"context"
	"errors"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	"git.at.oechsler.it/samuel/dash/v2/infra/persistence/model"

	"gorm.io/gorm"
)

var _ domainrepo.ApplicationRepository = (*GormApplicationRepo)(nil)

type GormApplicationRepo struct {
	db *gorm.DB
}

func NewGormApplicationRepo(db *gorm.DB) (*GormApplicationRepo, error) {
	if err := db.AutoMigrate(&model.Application{}); err != nil {
		return nil, err
	}
	return &GormApplicationRepo{db: db}, nil
}

func (r *GormApplicationRepo) Upsert(ctx context.Context, record *domainrepo.ApplicationRecord) error {
	m := &model.Application{
		Icon:            record.Icon,
		DisplayName:     record.DisplayName,
		Url:             record.Url,
		VisibleToGroups: record.VisibleToGroups,
	}
	if record.ID != 0 {
		m.ID = record.ID
	}
	return r.db.WithContext(ctx).Save(m).Error
}

func (r *GormApplicationRepo) Get(ctx context.Context, id uint) (*domainrepo.ApplicationRecord, error) {
	var app model.Application
	if err := r.db.WithContext(ctx).First(&app, id).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, domainerrors.NotFound(domainerrors.EntityApplication)
		}
		return nil, err
	}
	return &domainrepo.ApplicationRecord{
		ID:              app.ID,
		Icon:            app.Icon,
		DisplayName:     app.DisplayName,
		Url:             app.Url,
		VisibleToGroups: app.VisibleToGroups,
	}, nil
}

func (r *GormApplicationRepo) List(ctx context.Context) ([]domainrepo.ApplicationRecord, error) {
	var apps []model.Application
	if err := r.db.WithContext(ctx).Order("LOWER(display_name) ASC, id ASC").Find(&apps).Error; err != nil {
		return nil, err
	}
	records := make([]domainrepo.ApplicationRecord, len(apps))
	for i, app := range apps {
		records[i] = domainrepo.ApplicationRecord{
			ID:              app.ID,
			Icon:            app.Icon,
			DisplayName:     app.DisplayName,
			Url:             app.Url,
			VisibleToGroups: app.VisibleToGroups,
		}
	}
	return records, nil
}

func (r *GormApplicationRepo) Delete(ctx context.Context, id uint) error {
	return r.db.WithContext(ctx).Delete(&model.Application{}, id).Error
}
