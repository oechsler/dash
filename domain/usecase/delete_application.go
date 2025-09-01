package usecase

import (
	"context"
	"dash/data/repo"
	"fmt"
)

// DeleteApplication deletes an application by id. Applications are admin-managed,
// so there is no user-ownership check. Treat missing resource as not found/forbidden
// depending on existing conventions (for now, return ErrForbidden to avoid leaking info).
type DeleteApplication struct {
	ApplicationRepo repo.ApplicationRepo
}

func NewDeleteApplication(applicationRepo repo.ApplicationRepo) *DeleteApplication {
	return &DeleteApplication{ApplicationRepo: applicationRepo}
}

func (uc *DeleteApplication) Execute(ctx context.Context, id uint) error {
	if id == 0 {
		return fmt.Errorf("%w: %s", ErrValidation, "id is required")
	}

	app, err := uc.ApplicationRepo.Get(ctx, id)
	if err != nil {
		return err
	}
	if app == nil {
		return ErrForbidden // hide existence details
	}

	return uc.ApplicationRepo.Delete(ctx, id)
}
