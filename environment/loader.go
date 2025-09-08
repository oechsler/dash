package environment

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/joho/godotenv"
	"github.com/samber/lo"
)

const APP_ENV = "APP_ENV"

type Env struct{}

func NewEnv() *Env {
	e := &Env{}

	if err := godotenv.Load(".env"); err != nil {
		_ = godotenv.Load("../.env")
	}

	if err := godotenv.Overload(".env.local"); err != nil {
		_ = godotenv.Overload("../.env.local")
	}

	appEnvFilename := fmt.Sprintf(".env.%s", e.Get())
	if err := godotenv.Overload(appEnvFilename); err != nil {
		_ = godotenv.Overload(fmt.Sprintf("../%s", appEnvFilename))
	}

	if len(e.Get()) == 0 {
		_ = os.Setenv(APP_ENV, "prod")
	}

	return e
}

func (e *Env) Get() string {
	return os.Getenv(APP_ENV)
}

func (e *Env) IsDevelopment() bool {
	lowercaseEnvName := strings.ToLower(e.Get())
	return strings.Contains(lowercaseEnvName, "dev")
}

func (e *Env) String(key string, defaultValue ...string) string {
	value := os.Getenv(key)
	if len(value) == 0 && len(defaultValue) > 0 {
		return defaultValue[0]
	} else if len(value) == 0 {
		return ""
	}
	return value
}

func (e *Env) Int(key string, defaultValue ...int) int {
	value := e.String(key, lo.Map(defaultValue, func(dv int, _ int) string {
		return fmt.Sprintf("%d", dv)
	})...)
	if len(value) == 0 {
		return -1
	}
	intValue, err := strconv.Atoi(value)
	if err != nil {
		return -1
	}
	return intValue
}

func (e *Env) Bool(key string, defaultValue ...bool) bool {
	value := e.String(key, lo.Map(defaultValue, func(dv bool, _ int) string {
		return fmt.Sprintf("%t", dv)
	})...)
	if len(value) == 0 {
		return false
	}
	boolValue, err := strconv.ParseBool(value)
	if err != nil {
		return false
	}
	return boolValue
}
