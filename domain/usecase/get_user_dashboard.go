package usecase

import (
	"context"
	"dash/data/model"
	"dash/data/repo"
	domainmodel "dash/domain/model"
	"time"
)

type GetUserDashboard struct {
	DashboardRepo            repo.DashboardRepo
	GetUserCategories        *GetUserCategories
	GetUserApplications      *GetUserApplications
	GetUserDashboardGreeting *GetUserDashboardGreeting
}

func NewGetUserDashboard(
	dashboardRepo repo.DashboardRepo,
	getUserCategories *GetUserCategories,
	getUserApplications *GetUserApplications,
	getUserDashboardGreeting *GetUserDashboardGreeting,
) *GetUserDashboard {
	return &GetUserDashboard{
		DashboardRepo:            dashboardRepo,
		GetUserCategories:        getUserCategories,
		GetUserApplications:      getUserApplications,
		GetUserDashboardGreeting: getUserDashboardGreeting,
	}
}

func (uc *GetUserDashboard) Execute(
	ctx context.Context,
	userId string,
	userGroups []string,
	userFirstName string,
	localTime time.Time,
) (*domainmodel.Dashboard, error) {
	dashboard, err := uc.DashboardRepo.GetByUserId(ctx, userId)
	if err != nil {
		return nil, err
	}
	if dashboard == nil {
		if err := uc.DashboardRepo.Upsert(ctx, &model.Dashboard{
			UserId: userId,
		}); err != nil {
			return nil, err
		}

		dashboard, err = uc.DashboardRepo.GetByUserId(ctx, userId)
		if err != nil {
			return nil, err
		}
	}

	categories, err := uc.GetUserCategories.Execute(ctx, userId)
	if err != nil {
		return nil, err
	}

	apps, err := uc.GetUserApplications.Execute(ctx, userGroups)
	if err != nil {
		return nil, err
	}

	greeting, err := uc.GetUserDashboardGreeting.Execute(ctx, userFirstName, localTime)
	if err != nil {
		return nil, err
	}

	return &domainmodel.Dashboard{
		Applications: apps,
		Categories:   categories,
		Greeting:     greeting,
	}, nil
}
