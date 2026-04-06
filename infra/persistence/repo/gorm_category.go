package repo

import (
	"context"
	"errors"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	"github.com/oechsler-it/dash/infra/persistence/model"
	domainrepo "github.com/oechsler-it/dash/domain/repo"

	"gorm.io/gorm"
)

var _ domainrepo.CategoryRepository = (*GormCategoryRepo)(nil)

type GormCategoryRepo struct {
	db *gorm.DB
}

func NewGormCategoryRepo(db *gorm.DB) (*GormCategoryRepo, error) {
	if err := db.AutoMigrate(&model.Category{}); err != nil {
		return nil, err
	}
	return &GormCategoryRepo{db: db}, nil
}

func (r *GormCategoryRepo) Upsert(ctx context.Context, record *domainrepo.CategoryRecord) error {
	m := &model.Category{
		DashboardID: record.DashboardID,
		DisplayName: record.DisplayName,
		IsShelved:   record.IsShelved,
	}
	if record.ID != 0 {
		m.ID = record.ID
	}
	if err := r.db.WithContext(ctx).Save(m).Error; err != nil {
		return err
	}
	record.ID = m.ID
	return nil
}

func (r *GormCategoryRepo) Get(ctx context.Context, id uint) (*domainrepo.CategoryRecord, error) {
	var c model.Category
	if err := r.db.WithContext(ctx).First(&c, id).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, domainerrors.NotFound(domainerrors.EntityCategory)
		}
		return nil, err
	}
	return &domainrepo.CategoryRecord{
		ID:          c.ID,
		DashboardID: c.DashboardID,
		DisplayName: c.DisplayName,
		IsShelved:   c.IsShelved,
	}, nil
}

func (r *GormCategoryRepo) ListByDashboardID(ctx context.Context, dashboardID uint) ([]domainrepo.CategoryRecord, error) {
	var list []model.Category
	if err := r.db.WithContext(ctx).
		Where("dashboard_id = ?", dashboardID).
		Order("LOWER(display_name) ASC, id ASC").
		Find(&list).Error; err != nil {
		return nil, err
	}
	records := make([]domainrepo.CategoryRecord, len(list))
	for i, c := range list {
		records[i] = domainrepo.CategoryRecord{
			ID:          c.ID,
			DashboardID: c.DashboardID,
			DisplayName: c.DisplayName,
			IsShelved:   c.IsShelved,
		}
	}
	return records, nil
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
