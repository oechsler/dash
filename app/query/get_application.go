package query

import (
	"context"

	domainerrors "github.com/oechsler-it/dash/domain/errors"
	domainmodel "github.com/oechsler-it/dash/domain/model"
	domainrepo "github.com/oechsler-it/dash/domain/repo"
)

// ApplicationGetter handles the get-application query.
type ApplicationGetter interface {
	Handle(ctx context.Context, id uint) (*domainmodel.AppLink, error)
}

type GetApplication struct {
	ApplicationRepo domainrepo.ApplicationRepository
}

func NewGetApplication(applicationRepo domainrepo.ApplicationRepository) *GetApplication {
	return &GetApplication{ApplicationRepo: applicationRepo}
}

func (h *GetApplication) Handle(ctx context.Context, id uint) (*domainmodel.AppLink, error) {
	app, err := h.ApplicationRepo.Get(ctx, id)
	if err != nil {
		return nil, domainerrors.WrapRepo("get application", err)
	}
	icon, err := domainmodel.ParseIcon(app.Icon)
	if err != nil {
		return nil, domainerrors.Internal("get application: parse icon", err)
	}
	appUrl, err := domainmodel.ParseBookmarkURL(app.Url)
	if err != nil {
		return nil, domainerrors.Internal("get application: parse url", err)
	}
	return &domainmodel.AppLink{
		ID:              app.ID,
		Icon:            icon,
		DisplayName:     app.DisplayName,
		Url:             appUrl,
		VisibleToGroups: app.VisibleToGroups,
	}, nil
}
