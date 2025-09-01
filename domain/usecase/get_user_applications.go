package usecase

import (
	"context"
	domainmodel "dash/domain/model"

	"github.com/samber/lo"
)

type GetUserApplications struct {
	ListApplications *ListApplications
}

func NewGetUserApplications(listApplications *ListApplications) *GetUserApplications {
	return &GetUserApplications{
		ListApplications: listApplications,
	}
}

func (uc *GetUserApplications) Execute(ctx context.Context, groupsOfUser []string) ([]domainmodel.AppLink, error) {
	applications, err := uc.ListApplications.Execute(ctx)
	if err != nil {
		return nil, err
	}

	filtered := lo.Filter(applications, func(app domainmodel.AppLink, _ int) bool {
		if len(app.VisibleToGroups) == 0 {
			return true
		}
		return lo.ContainsBy(groupsOfUser, func(group string) bool {
			return lo.Contains(app.VisibleToGroups, group)
		})
	})

	return filtered, nil
}
