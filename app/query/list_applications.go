package query

import (
	"context"

	domainerrors "git.at.oechsler.it/samuel/dash/v2/domain/errors"
	domainmodel "git.at.oechsler.it/samuel/dash/v2/domain/model"
	domainrepo "git.at.oechsler.it/samuel/dash/v2/domain/repo"
)

// ApplicationsLister handles the list-applications query.
type ApplicationsLister interface {
	Handle(ctx context.Context) ([]domainmodel.AppLink, error)
}

type ListApplications struct {
	ApplicationRepo domainrepo.ApplicationRepository
}

func NewListApplications(applicationRepo domainrepo.ApplicationRepository) *ListApplications {
	return &ListApplications{ApplicationRepo: applicationRepo}
}

func (h *ListApplications) Handle(ctx context.Context) ([]domainmodel.AppLink, error) {
	applications, err := h.ApplicationRepo.List(ctx)
	if err != nil {
		return nil, domainerrors.Internal("list applications", err)
	}
	result := make([]domainmodel.AppLink, 0, len(applications))
	for _, a := range applications {
		icon, err := domainmodel.ParseIcon(a.Icon)
		if err != nil {
			return nil, domainerrors.Internal("list applications: parse icon", err)
		}
		appUrl, err := domainmodel.ParseBookmarkURL(a.Url)
		if err != nil {
			return nil, domainerrors.Internal("list applications: parse url", err)
		}
		result = append(result, domainmodel.AppLink{
			ID:              a.ID,
			Icon:            icon,
			DisplayName:     a.DisplayName,
			Url:             appUrl,
			VisibleToGroups: a.VisibleToGroups,
		})
	}
	return result, nil
}
