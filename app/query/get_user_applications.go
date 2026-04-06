package query

import (
	"context"

	domainmodel "github.com/oechsler-it/dash/domain/model"
	"github.com/oechsler-it/dash/domain/service"
)

// UserApplicationsGetter handles the get-user-applications query.
type UserApplicationsGetter interface {
	Handle(ctx context.Context, groupsOfUser []string) ([]domainmodel.AppLink, error)
}

type GetUserApplications struct {
	ListApplications *ListApplications
}

func NewGetUserApplications(listApplications *ListApplications) *GetUserApplications {
	return &GetUserApplications{ListApplications: listApplications}
}

func (h *GetUserApplications) Handle(ctx context.Context, groupsOfUser []string) ([]domainmodel.AppLink, error) {
	applications, err := h.ListApplications.Handle(ctx)
	if err != nil {
		return nil, err
	}
	return service.FilterForUser(applications, groupsOfUser), nil
}
