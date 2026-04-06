package config

import (
	"errors"
	"os"

	"github.com/ilyakaznacheev/cleanenv"
)

const configFile = "/etc/dash/config.yaml"

func Load() (*Config, error) {
	cfg := &Config{}

	if _, err := os.Stat(configFile); err == nil {
		return cfg, cleanenv.ReadConfig(configFile, cfg)
	}

	if err := cleanenv.ReadEnv(cfg); err != nil {
		return nil, errors.New("failed to load config from environment: " + err.Error())
	}

	return cfg, nil
}
