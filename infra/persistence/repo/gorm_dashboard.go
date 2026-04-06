package repo

import (
	"context"
	"errors"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	"git.at.oechsler.it/samuel/dash/v2/infra/persistence/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"

	"gorm.io/gorm"
)

var _ domainrepo.DashboardRepository = (*GormDashboardRepo)(nil)

type GormDashboardRepo struct {
	db *gorm.DB
}

func NewGormDashboardRepo(db *gorm.DB) (*GormDashboardRepo, error) {
	if err := db.AutoMigrate(&model.Dashboard{}); err != nil {
		return nil, err
	}
	return &GormDashboardRepo{db: db}, nil
}

func (r *GormDashboardRepo) Upsert(ctx context.Context, record *domainrepo.DashboardRecord) error {
	m := &model.Dashboard{
		UserId: record.UserID,
	}
	if record.ID != 0 {
		m.ID = record.ID
	}
	return r.db.WithContext(ctx).Save(m).Error
}

func (r *GormDashboardRepo) Get(ctx context.Context, id uint) (*domainrepo.DashboardRecord, error) {
	var dashboard model.Dashboard
	err := r.db.WithContext(ctx).First(&dashboard, id).Error
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, domainerrors.NotFound(domainerrors.EntityDashboard)
		}
		return nil, err
	}
	return &domainrepo.DashboardRecord{ID: dashboard.ID, UserID: dashboard.UserId}, nil
}

func (r *GormDashboardRepo) GetByUserID(ctx context.Context, userID string) (*domainrepo.DashboardRecord, error) {
	var dashboard model.Dashboard
	err := r.db.WithContext(ctx).Where("user_id = ?", userID).First(&dashboard).Error
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, domainerrors.NotFound(domainerrors.EntityDashboard)
		}
		return nil, err
	}
	return &domainrepo.DashboardRecord{ID: dashboard.ID, UserID: dashboard.UserId}, nil
}

func (r *GormDashboardRepo) Delete(ctx context.Context, id uint) error {
	return r.db.WithContext(ctx).Delete(&model.Dashboard{}, id).Error
}
