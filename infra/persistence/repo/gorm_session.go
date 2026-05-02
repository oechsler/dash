package repo

import (
	"context"
	"encoding/json"
	"time"

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
	// Copy data from the legacy pinned_sessions table (kept intact for downgrade
	// safety) into the new sessions table on first run. AutoMigrate creates the
	// sessions table before the INSERT so the sequence is safe.
	//
	// TODO(v3): drop this migration block once all deployments have run at least
	// once after upgrading to this version. The pinned_sessions table itself can
	// also be dropped at that point.
	if err := db.AutoMigrate(&model.Session{}); err != nil {
		return nil, err
	}
	if err := db.Exec(`
		INSERT INTO sessions
		SELECT * FROM pinned_sessions
		WHERE NOT EXISTS (SELECT 1 FROM sessions)
		  AND EXISTS (
		        SELECT 1 FROM information_schema.tables
		        WHERE table_name = 'pinned_sessions'
		      )
	`).Error; err != nil {
		return nil, err
	}
	return &GormSessionRepo{db: db}, nil
}

func (r *GormSessionRepo) Create(ctx context.Context, record *domainrepo.SessionRecord) error {
	m := &model.Session{
		ID:          record.ID,
		UserID:      record.UserID,
		SessionID:   record.SessionID,
		IssuedAt:    record.IssuedAt,
		ExpiresAt:   record.ExpiresAt,
		LastIP:      record.LastIP,
		UserAgent:   record.UserAgent,
		Sub:         record.Sub,
		Username:    record.Username,
		Email:       record.Email,
		FirstName:   record.FirstName,
		LastName:    record.LastName,
		DisplayName: record.DisplayName,
		Picture:     record.Picture,
		ProfileUrl:  record.ProfileUrl,
		Groups:      encodeGroups(record.Groups),
		IsAdmin:     record.IsAdmin,
	}
	return r.db.WithContext(ctx).Clauses(clause.OnConflict{DoNothing: true}).Create(m).Error
}

func (r *GormSessionRepo) Pin(ctx context.Context, sessionID string, userID string, pinnedUntil time.Time) error {
	return r.db.WithContext(ctx).
		Model(&model.Session{}).
		Where("session_id = ? AND user_id = ?", sessionID, userID).
		Update("pinned_until", pinnedUntil).Error
}

func (r *GormSessionRepo) Unpin(ctx context.Context, recordID string, userID string) error {
	return r.db.WithContext(ctx).
		Model(&model.Session{}).
		Where("id = ? AND user_id = ?", recordID, userID).
		Update("pinned_until", time.Time{}).Error
}

// Touch updates access metadata and returns the full record if the session is still
// active (expires_at > now OR pinned_until > now). Returns nil if the session is
// not found or has fully expired.
func (r *GormSessionRepo) Touch(ctx context.Context, sessionID string, lastIP string, userAgent string) (*domainrepo.SessionRecord, error) {
	now := time.Now()
	var m model.Session
	tx := r.db.WithContext(ctx).Raw(`
		UPDATE sessions
		SET last_ip          = ?,
		    user_agent       = ?,
		    last_accessed_at = ?,
		    updated_at       = ?,
		    pinned_until     = CASE WHEN pinned_until > NOW()
		                           THEN NOW() + INTERVAL '1 year'
		                           ELSE pinned_until
		                      END
		WHERE session_id = ?
		  AND (expires_at > NOW() OR pinned_until > NOW())
		RETURNING *`,
		lastIP, userAgent, now, now, sessionID,
	).Scan(&m)
	if tx.Error != nil {
		return nil, tx.Error
	}
	if tx.RowsAffected == 0 {
		return nil, nil // session not found or fully expired
	}
	return toSessionRecord(&m), nil
}

func (r *GormSessionRepo) ListByUserID(ctx context.Context, userID string) ([]*domainrepo.SessionRecord, error) {
	var ms []model.Session
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
		records = append(records, toSessionRecord(&ms[i]))
	}
	return records, nil
}

func (r *GormSessionRepo) DeleteByID(ctx context.Context, recordID string, userID string) error {
	return r.db.WithContext(ctx).Where("id = ? AND user_id = ?", recordID, userID).Delete(&model.Session{}).Error
}

func (r *GormSessionRepo) DeleteBySessionID(ctx context.Context, sessionID string) error {
	return r.db.WithContext(ctx).Where("session_id = ?", sessionID).Delete(&model.Session{}).Error
}

func (r *GormSessionRepo) DeleteByUserID(ctx context.Context, userID string) error {
	return r.db.WithContext(ctx).Where("user_id = ?", userID).Delete(&model.Session{}).Error
}

func (r *GormSessionRepo) RefreshBySessionID(ctx context.Context, record *domainrepo.SessionRecord) error {
	return r.db.WithContext(ctx).
		Model(&model.Session{}).
		Where("session_id = ?", record.SessionID).
		Updates(map[string]any{
			"issued_at":    record.IssuedAt,
			"expires_at":   record.ExpiresAt,
			"sub":          record.Sub,
			"username":     record.Username,
			"email":        record.Email,
			"first_name":   record.FirstName,
			"last_name":    record.LastName,
			"display_name": record.DisplayName,
			"picture":      record.Picture,
			"profile_url":  record.ProfileUrl,
			"groups":       encodeGroups(record.Groups),
			"is_admin":     record.IsAdmin,
		}).Error
}

func (r *GormSessionRepo) DeleteExpired(ctx context.Context) error {
	now := time.Now()
	return r.db.WithContext(ctx).
		Where("expires_at < ? AND pinned_until < ?", now, now).
		Delete(&model.Session{}).Error
}

func toSessionRecord(m *model.Session) *domainrepo.SessionRecord {
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
		Groups:         decodeGroups(m.Groups),
		IsAdmin:        m.IsAdmin,
	}
}

func encodeGroups(groups []string) string {
	if len(groups) == 0 {
		return ""
	}
	b, _ := json.Marshal(groups)
	return string(b)
}

func decodeGroups(s string) []string {
	if s == "" {
		return nil
	}
	var groups []string
	_ = json.Unmarshal([]byte(s), &groups)
	return groups
}
