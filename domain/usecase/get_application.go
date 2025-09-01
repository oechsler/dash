package usecase

import (
	"context"
	"dash/data/repo"
	domainmodel "dash/domain/model"
)

type GetApplication struct {
	ApplicationRepo repo.ApplicationRepo
}

func NewGetApplication(applicationRepo repo.ApplicationRepo) *GetApplication {
	return &GetApplication{ApplicationRepo: applicationRepo}
}

func (uc *GetApplication) Execute(ctx context.Context, id uint) (*domainmodel.AppLink, error) {
	app, err := uc.ApplicationRepo.Get(ctx, id)
	if err != nil {
		return nil, err
	}
	if app == nil {
		return nil, nil
	}
	return &domainmodel.AppLink{
		ID:              app.ID,
		Icon:            app.Icon,
		DisplayName:     app.DisplayName,
		Description:     app.Description,
		Url:             app.Url,
		VisibleToGroups: app.VisibleToGroups,
	}, nil
}
