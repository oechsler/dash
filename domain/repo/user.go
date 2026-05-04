package repo

import "context"

// UserRepository manages the internal user records that act as the relational
// anchor for all user-owned data. User records are created implicitly via
// IdpLinkRepository.ResolveOrCreate and deleted explicitly to cascade all
// associated data (dashboards, settings, themes, sessions, idp_links).
type UserRepository interface {
	// DeleteByID removes the user record. All associated data is deleted via
	// ON DELETE CASCADE constraints on the dependent tables.
	DeleteByID(ctx context.Context, id string) error
}
