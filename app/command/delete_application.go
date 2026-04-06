package command

import (
	"context"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainrepo "github.com/oechsler-it/dash/domain/repo"
)

// ApplicationDeleter handles the delete-application command.
// Applications are admin-managed, so there is no user-ownership check.
type ApplicationDeleter interface {
	Handle(ctx context.Context, id uint) error
}

type DeleteApplication struct {
	ApplicationRepo domainrepo.ApplicationRepository
}

func NewDeleteApplication(applicationRepo domainrepo.ApplicationRepository) *DeleteApplication {
	return &DeleteApplication{ApplicationRepo: applicationRepo}
}

func (h *DeleteApplication) Handle(ctx context.Context, id uint) error {
	if id == 0 {
		return domainerrors.Validation(domainerrors.Violation{Message: "id is required"})
	}

	_, err := h.ApplicationRepo.Get(ctx, id)
	if err != nil {
		return domainerrors.WrapRepo("delete application: get", err)
	}

	if err := h.ApplicationRepo.Delete(ctx, id); err != nil {
		return domainerrors.Internal("delete application: delete", err)
	}
	return nil
}
