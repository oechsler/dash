package service

import (
	"github.com/oechsler-it/dash/domain/model"

	"github.com/samber/lo"
)

// FilterForUser returns only the applications visible to a user based on group membership.
// Business rule: an application with no groups is visible to everyone.
func FilterForUser(apps []model.AppLink, userGroups []string) []model.AppLink {
	return lo.Filter(apps, func(app model.AppLink, _ int) bool {
		if len(app.VisibleToGroups) == 0 {
			return true
		}
		return lo.ContainsBy(userGroups, func(group string) bool {
			return lo.Contains(app.VisibleToGroups, group)
		})
	})
}
