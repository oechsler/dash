package usecase

import (
	"context"
	"dash/domain/model"
	"fmt"
	"time"
)

type GetUserDashboardGreeting struct{}

func NewGetUserDashboardGreeting() *GetUserDashboardGreeting {
	return &GetUserDashboardGreeting{}
}

func (uc *GetUserDashboardGreeting) Execute(ctx context.Context, userFirstName string, localTime time.Time) (*model.DashboardGreeting, error) {
	var now time.Time
	if localTime.IsZero() {
		now = time.Now()
	} else {
		now = localTime
	}

	local := now.Local()
	hour := local.Hour()

	var greet string
	switch {
	case hour >= 5 && hour < 12:
		greet = "Good morning"
	case hour >= 12 && hour < 17:
		greet = "Good afternoon"
	case hour >= 17 && hour < 22:
		greet = "Good evening"
	case hour >= 22 && hour <= 23 || hour >= 0 && hour < 5:
		greet = "Good night"
	default:
		greet = "Hello"
	}

	msg := fmt.Sprintf("%s, %s!", greet, userFirstName)

	return &model.DashboardGreeting{
		Date:    local.Format("Monday, 02 January 2006"),
		Message: msg,
	}, nil
}
