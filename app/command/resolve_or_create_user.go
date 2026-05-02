package command

import (
	"context"

	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// UserResolver resolves or creates an internal UserID for an OIDC identity.
// isNew is true when a new link was just created — the caller should then
// migrate any pre-existing data from sub to the returned UserID.
type UserResolver interface {
	Handle(ctx context.Context, issuer, sub string) (userID string, isNew bool, err error)
}

type ResolveOrCreateUser struct {
	repo domainrepo.IdpLinkRepository
}

func NewResolveOrCreateUser(repo domainrepo.IdpLinkRepository) *ResolveOrCreateUser {
	return &ResolveOrCreateUser{repo: repo}
}

func (h *ResolveOrCreateUser) Handle(ctx context.Context, issuer, sub string) (string, bool, error) {
	return h.repo.ResolveOrCreate(ctx, issuer, sub)
}
