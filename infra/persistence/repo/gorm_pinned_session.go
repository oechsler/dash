package repo

import (
	"context"
	"encoding/json"
	"errors"
	"time"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
	"git.at.oechsler.it/samuel/dash/v2/infra/persistence/model"

	"gorm.io/gorm"
	"gorm.io/gorm/clause"
)

var _ domainrepo.SessionRepository = (*GormSessionRepo)(nil)

type GormSessionRepo struct {
	db *gorm.DB
}

func NewGormSessionRepo(db *gorm.DB) (*GormSessionRepo, error) {
	if err := db.AutoMigrate(&model.PinnedSession{}); err != nil {
		return nil, err
	}
	return &GormSessionRepo{db: db}, nil
}

func (r *GormSessionRepo) Create(ctx context.Context, record *domainrepo.SessionRecord) error {
	groups, err := json.Marshal(record.Groups)
	if err != nil {
		return err
	}
	m := &model.PinnedSession{
		ID:             record.ID,
		UserID:         record.UserID,
		SessionID:      record.SessionID,
		IssuedAt:       record.IssuedAt,
		ExpiresAt:      record.ExpiresAt,
		PinnedUntil:    record.PinnedUntil,
		LastAccessedAt: record.LastAccessedAt,
		LastIP:         record.LastIP,
		UserAgent:      record.UserAgent,
		Sub:            record.Sub,
		Username:       record.Username,
		Email:          record.Email,
		FirstName:      record.FirstName,
		LastName:       record.LastName,
		DisplayName:    record.DisplayName,
		Picture:        record.Picture,
		ProfileUrl:     record.ProfileUrl,
		Groups:         string(groups),
		IsAdmin:        record.IsAdmin,
	}
	return r.db.WithContext(ctx).Clauses(clause.OnConflict{DoNothing: true}).Create(m).Error
}

func (r *GormSessionRepo) Pin(ctx context.Context, sessionID string, userID string, pinnedUntil time.Time) error {
	return r.db.WithContext(ctx).
		Model(&model.PinnedSession{}).
		Where("session_id = ? AND user_id = ?", sessionID, userID).
		Update("pinned_until", pinnedUntil).Error
}

func (r *GormSessionRepo) Unpin(ctx context.Context, id string, userID string) error {
	return r.db.WithContext(ctx).
		Model(&model.PinnedSession{}).
		Where("id = ? AND user_id = ?", id, userID).
		Update("pinned_until", time.Time{}).Error
}

// GetBySessionID returns the session only if it is actively pinned (PinnedUntil > now).
// Used exclusively for pinned-fallback auth after OIDC token expiry.
func (r *GormSessionRepo) GetBySessionID(ctx context.Context, sessionID string) (*domainrepo.SessionRecord, error) {
	var m model.PinnedSession
	err := r.db.WithContext(ctx).
		Where("session_id = ? AND pinned_until > ?", sessionID, time.Now()).
		First(&m).Error
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, domainerrors.NotFound(domainerrors.EntityPinnedSession)
		}
		return nil, err
	}
	return toSessionRecord(&m)
}

// ExistsBySessionID returns true if any record with the given SessionID exists,
// regardless of pin status or expiry. Used for session revocation checks.
func (r *GormSessionRepo) ExistsBySessionID(ctx context.Context, sessionID string) (bool, error) {
	var count int64
	err := r.db.WithContext(ctx).
		Model(&model.PinnedSession{}).
		Where("session_id = ?", sessionID).
		Count(&count).Error
	return count > 0, err
}

func (r *GormSessionRepo) Touch(ctx context.Context, sessionID string, lastIP string, userAgent string) (bool, time.Time, error) {
	now := time.Now()
	// Extend PinnedUntil by 1 year when the session is actively pinned (sliding window).
	// The RETURNING clause gives us the new value so the caller knows whether to refresh
	// the browser cookie.
	var row struct {
		PinnedUntil time.Time `gorm:"column:pinned_until"`
	}
	tx := r.db.WithContext(ctx).Raw(`
		UPDATE pinned_sessions
		SET last_ip          = ?,
		    user_agent       = ?,
		    last_accessed_at = ?,
		    updated_at       = ?,
		    pinned_until     = CASE WHEN pinned_until > NOW()
		                           THEN NOW() + INTERVAL '1 year'
		                           ELSE pinned_until
		                      END
		WHERE session_id = ?
		RETURNING pinned_until`,
		lastIP, userAgent, now, now, sessionID,
	).Scan(&row)
	if tx.Error != nil {
		return false, time.Time{}, tx.Error
	}
	return tx.RowsAffected > 0, row.PinnedUntil, nil
}

func (r *GormSessionRepo) TouchBySessionID(ctx context.Context, sessionID string, newPinnedUntil time.Time, lastIP string, userAgent string) error {
	return r.db.WithContext(ctx).
		Model(&model.PinnedSession{}).
		Where("session_id = ?", sessionID).
		Updates(map[string]any{
			"pinned_until":     newPinnedUntil,
			"last_accessed_at": time.Now(),
			"last_ip":          lastIP,
			"user_agent":       userAgent,
		}).Error
}

func (r *GormSessionRepo) ListByUserID(ctx context.Context, userID string) ([]*domainrepo.SessionRecord, error) {
	var ms []model.PinnedSession
	now := time.Now()
	err := r.db.WithContext(ctx).
		Where("user_id = ? AND (expires_at > ? OR pinned_until > ?)", userID, now, now).
		Order("created_at DESC").
		Find(&ms).Error
	if err != nil {
		return nil, err
	}
	records := make([]*domainrepo.SessionRecord, 0, len(ms))
	for i := range ms {
		rec, err := toSessionRecord(&ms[i])
		if err != nil {
			return nil, err
		}
		records = append(records, rec)
	}
	return records, nil
}

func (r *GormSessionRepo) DeleteByID(ctx context.Context, id string, userID string) error {
	return r.db.WithContext(ctx).Where("id = ? AND user_id = ?", id, userID).Delete(&model.PinnedSession{}).Error
}

func (r *GormSessionRepo) DeleteBySessionID(ctx context.Context, sessionID string) error {
	return r.db.WithContext(ctx).Where("session_id = ?", sessionID).Delete(&model.PinnedSession{}).Error
}

func (r *GormSessionRepo) DeleteByUserID(ctx context.Context, userID string) error {
	return r.db.WithContext(ctx).Where("user_id = ?", userID).Delete(&model.PinnedSession{}).Error
}

func (r *GormSessionRepo) DeleteExpired(ctx context.Context) error {
	now := time.Now()
	return r.db.WithContext(ctx).
		Where("expires_at < ? AND pinned_until < ?", now, now).
		Delete(&model.PinnedSession{}).Error
}

func toSessionRecord(m *model.PinnedSession) (*domainrepo.SessionRecord, error) {
	var groups []string
	if m.Groups != "" {
		if err := json.Unmarshal([]byte(m.Groups), &groups); err != nil {
			return nil, err
		}
	}
	return &domainrepo.SessionRecord{
		ID:             m.ID,
		UserID:         m.UserID,
		SessionID:      m.SessionID,
		IssuedAt:       m.IssuedAt,
		ExpiresAt:      m.ExpiresAt,
		PinnedUntil:    m.PinnedUntil,
		LastAccessedAt: m.LastAccessedAt,
		LastIP:         m.LastIP,
		UserAgent:      m.UserAgent,
		CreatedAt:      m.CreatedAt,
		Sub:            m.Sub,
		Username:       m.Username,
		Email:          m.Email,
		FirstName:      m.FirstName,
		LastName:       m.LastName,
		DisplayName:    m.DisplayName,
		Picture:        m.Picture,
		ProfileUrl:     m.ProfileUrl,
		Groups:         groups,
		IsAdmin:        m.IsAdmin,
	}, nil
}
