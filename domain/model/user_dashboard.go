package model

// UserDashboard is the aggregate root for a user's personal dashboard.
// It enforces ownership semantics for categories and bookmarks.
// On the write path, load it from a DashboardRecord and use its methods
// to express ownership checks as domain concepts instead of raw comparisons.
type UserDashboard struct {
	id     uint
	userID string
}

// NewUserDashboard constructs a UserDashboard from a dashboard ID and user ID.
func NewUserDashboard(id uint, userID string) UserDashboard {
	return UserDashboard{id: id, userID: userID}
}

// ID returns the internal dashboard identifier.
func (d UserDashboard) ID() uint { return d.id }

// UserID returns the ID of the user who owns this dashboard.
func (d UserDashboard) UserID() string { return d.userID }

// OwnedBy reports whether this dashboard belongs to the given user.
func (d UserDashboard) OwnedBy(userID string) bool { return d.userID == userID }

// OwnsCategory reports whether a category with the given dashboardID belongs to this dashboard.
func (d UserDashboard) OwnsCategory(dashboardID uint) bool { return dashboardID == d.id }
