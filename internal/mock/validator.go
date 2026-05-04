package mock

import "github.com/stretchr/testify/mock"

type Validator struct{ mock.Mock }

func (m *Validator) Struct(s any) error {
	return m.Called(s).Error(0)
}
