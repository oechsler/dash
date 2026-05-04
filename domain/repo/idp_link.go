package repo

import "context"

// IdpLinkRepository maps (issuer, sub) pairs to internal user IDs.
// This is the foundation for multi-IdP support: a user can link multiple
// IdP identities to a single internal account.
type IdpLinkRepository interface {
	// ResolveOrCreate looks up the internal UserID for (issuer, sub).
	// If no link exists yet, creates one with a UUID v5 derived from (issuer, sub)
	// and returns isNew=true so the caller can migrate any pre-existing data.
	ResolveOrCreate(ctx context.Context, issuer, sub string) (userID string, isNew bool, err error)
	// DeleteByUserID removes all IdP links for the given user.
	// Must be called when the user's account data is deleted.
	DeleteByUserID(ctx context.Context, userID string) error
}
