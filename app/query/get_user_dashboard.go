package query

import (
	"context"
	"errors"
	"time"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// UserDashboardGetter handles the get-user-dashboard query.
type UserDashboardGetter interface {
	Handle(ctx context.Context, userId string, userGroups []string, userFirstName string, localTime time.Time) (*domainmodel.Dashboard, error)
}

type GetUserDashboard struct {
	DashboardRepo       domainrepo.DashboardRepository
	GetUserCategories   *GetUserCategories
	GetUserApplications *GetUserApplications
}

func NewGetUserDashboard(
	dashboardRepo domainrepo.DashboardRepository,
	getUserCategories *GetUserCategories,
	getUserApplications *GetUserApplications,
) *GetUserDashboard {
	return &GetUserDashboard{
		DashboardRepo:       dashboardRepo,
		GetUserCategories:   getUserCategories,
		GetUserApplications: getUserApplications,
	}
}

func (h *GetUserDashboard) Handle(
	ctx context.Context,
	userId string,
	userGroups []string,
	userFirstName string,
	localTime time.Time,
) (*domainmodel.Dashboard, error) {
	_, err := h.DashboardRepo.GetByUserID(ctx, userId)
	if err != nil {
		var nfe *domainerrors.NotFoundError
		if !errors.As(err, &nfe) {
			return nil, domainerrors.Internal("get user dashboard: get dashboard", err)
		}
		// dashboard doesn't exist yet, auto-provision
		if err := h.DashboardRepo.Upsert(ctx, &domainrepo.DashboardRecord{
			UserID: userId,
		}); err != nil {
			return nil, domainerrors.Internal("get user dashboard: upsert dashboard", err)
		}
	}

	categories, err := h.GetUserCategories.Handle(ctx, userId)
	if err != nil {
		return nil, err
	}

	apps, err := h.GetUserApplications.Handle(ctx, userGroups)
	if err != nil {
		return nil, err
	}

	return &domainmodel.Dashboard{
		Applications: apps,
		Categories:   categories,
	}, nil
}
