package repo

import (
	"context"
	"dash/data/model"
	"errors"

	"gorm.io/gorm"
)

type BookmarkRepo interface {
	Upsert(ctx context.Context, bookmark *model.Bookmark) error
	Get(ctx context.Context, id uint) (*model.Bookmark, error)
	ListByDashboardID(ctx context.Context, dashboardID uint) ([]model.Bookmark, error)
	ListByCategoryIDs(ctx context.Context, categoryIDs []uint) ([]model.Bookmark, error)
	Delete(ctx context.Context, id uint) error
}

type GormBookmarkRepo struct {
	db *gorm.DB
}

func NewGormBookmarkRepo(db *gorm.DB) (*GormBookmarkRepo, error) {
	if err := db.AutoMigrate(&model.Bookmark{}); err != nil {
		return nil, err
	}
	return &GormBookmarkRepo{db: db}, nil
}

func (r *GormBookmarkRepo) Upsert(ctx context.Context, bookmark *model.Bookmark) error {
	return r.db.WithContext(ctx).Save(bookmark).Error
}

func (r *GormBookmarkRepo) Get(ctx context.Context, id uint) (*model.Bookmark, error) {
	var b model.Bookmark
	if err := r.db.WithContext(ctx).First(&b, id).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, nil
		}
		return nil, err
	}
	return &b, nil
}

func (r *GormBookmarkRepo) ListByDashboardID(ctx context.Context, dashboardID uint) ([]model.Bookmark, error) {
	var list []model.Bookmark
	if err := r.db.WithContext(ctx).
		Joins("JOIN categories ON categories.id = bookmarks.category_id").
		Where("categories.dashboard_id = ?", dashboardID).
		Order("bookmarks.display_name COLLATE NOCASE ASC, bookmarks.id ASC").
		Find(&list).Error; err != nil {
		return nil, err
	}
	return list, nil
}

func (r *GormBookmarkRepo) ListByCategoryIDs(ctx context.Context, categoryIDs []uint) ([]model.Bookmark, error) {
	var list []model.Bookmark
	if len(categoryIDs) == 0 {
		return list, nil
	}
	if err := r.db.WithContext(ctx).
		Where("category_id IN ?", categoryIDs).
		Order("display_name COLLATE NOCASE ASC, id ASC").
		Find(&list).Error; err != nil {
		return nil, err
	}
	return list, nil
}

func (r *GormBookmarkRepo) Delete(ctx context.Context, id uint) error {
	return r.db.WithContext(ctx).Delete(&model.Bookmark{}, id).Error
}
