package persistence

import (
	"log"

	"git.at.oechsler.it/samuel/dash/v2/config"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

func NewDB(cfg *config.DatabaseConfig) (*gorm.DB, error) {
	log.Printf("connecting to database (postgres)")
	return gorm.Open(postgres.Open(cfg.URL), &gorm.Config{
		PrepareStmt: true,
	})
}
