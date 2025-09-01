package usecase

import (
	"context"
	"dash/data/model"
	"dash/data/repo"
	domainmodel "dash/domain/model"

	"github.com/samber/lo"
)

type ListApplications struct {
	ApplicationRepo repo.ApplicationRepo
}

func NewListApplications(applicationRepo repo.ApplicationRepo) *ListApplications {
	return &ListApplications{ApplicationRepo: applicationRepo}
}

func (uc *ListApplications) Execute(ctx context.Context) ([]domainmodel.AppLink, error) {
	applications, err := uc.ApplicationRepo.List(ctx)
	if err != nil {
		return nil, err
	}
	return lo.Map(applications, func(a model.Application, _ int) domainmodel.AppLink {
		return domainmodel.AppLink{
			ID:              a.ID,
			Icon:            a.Icon,
			DisplayName:     a.DisplayName,
			Description:     a.Description,
			Url:             a.Url,
			VisibleToGroups: a.VisibleToGroups,
		}
	}), nil
}
