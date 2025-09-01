package usecase

import (
	"context"
	"dash/data/repo"
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
		return ValidationMsg("id is required")
	}

	app, err := uc.ApplicationRepo.Get(ctx, id)
	if err != nil {
		return Internal("delete application: get", err)
	}
	if app == nil {
		return Forbidden("application not accessible", nil)
	}

	if err := uc.ApplicationRepo.Delete(ctx, id); err != nil {
		return Internal("delete application: delete", err)
	}
	return nil
}
