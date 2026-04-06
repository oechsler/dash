package repo

import (
	"context"
	"errors"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	"github.com/oechsler-it/dash/infra/persistence/model"
	domainrepo "github.com/oechsler-it/dash/domain/repo"

	"gorm.io/gorm"
)

var _ domainrepo.BookmarkRepository = (*GormBookmarkRepo)(nil)

type GormBookmarkRepo struct {
	db *gorm.DB
}

func NewGormBookmarkRepo(db *gorm.DB) (*GormBookmarkRepo, error) {
	if err := db.AutoMigrate(&model.Bookmark{}); err != nil {
		return nil, err
	}
	return &GormBookmarkRepo{db: db}, nil
}

func (r *GormBookmarkRepo) Upsert(ctx context.Context, record *domainrepo.BookmarkRecord) error {
	m := &model.Bookmark{
		CategoryID:  record.CategoryID,
		Icon:        record.Icon,
		DisplayName: record.DisplayName,
		Url:         record.Url,
	}
	if record.ID != 0 {
		m.ID = record.ID
	}
	return r.db.WithContext(ctx).Save(m).Error
}

func (r *GormBookmarkRepo) Get(ctx context.Context, id uint) (*domainrepo.BookmarkRecord, error) {
	var b model.Bookmark
	if err := r.db.WithContext(ctx).First(&b, id).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, domainerrors.NotFound(domainerrors.EntityBookmark)
		}
		return nil, err
	}
	return &domainrepo.BookmarkRecord{
		ID:          b.ID,
		CategoryID:  b.CategoryID,
		Icon:        b.Icon,
		DisplayName: b.DisplayName,
		Url:         b.Url,
	}, nil
}

func (r *GormBookmarkRepo) ListByCategoryIDs(ctx context.Context, categoryIDs []uint) ([]domainrepo.BookmarkRecord, error) {
	var list []model.Bookmark
	if len(categoryIDs) == 0 {
		return []domainrepo.BookmarkRecord{}, nil
	}
	if err := r.db.WithContext(ctx).
		Where("category_id IN ?", categoryIDs).
		Order("LOWER(display_name) ASC, id ASC").
		Find(&list).Error; err != nil {
		return nil, err
	}
	records := make([]domainrepo.BookmarkRecord, len(list))
	for i, b := range list {
		records[i] = domainrepo.BookmarkRecord{
			ID:          b.ID,
			CategoryID:  b.CategoryID,
			Icon:        b.Icon,
			DisplayName: b.DisplayName,
			Url:         b.Url,
		}
	}
	return records, nil
}

func (r *GormBookmarkRepo) Delete(ctx context.Context, id uint) error {
	return r.db.WithContext(ctx).Delete(&model.Bookmark{}, id).Error
}
