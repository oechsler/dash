package main

import (
	"context"
	"log"
	"os"
	"os/signal"
	"syscall"
	"time"

	"git.at.oechsler.it/samuel/dash/v2/app"
	"git.at.oechsler.it/samuel/dash/v2/app/validation"
	"git.at.oechsler.it/samuel/dash/v2/config"
	"git.at.oechsler.it/samuel/dash/v2/delivery/web/handler"
	webi18n "git.at.oechsler.it/samuel/dash/v2/delivery/web/i18n"
	"git.at.oechsler.it/samuel/dash/v2/infra/oidc"
	"git.at.oechsler.it/samuel/dash/v2/infra/persistence"

	web "git.at.oechsler.it/samuel/dash/v2/delivery/web"
	"github.com/gofiber/fiber/v3"
)

// Build-time variables injected via -ldflags.
var (
	version   = "dev"
	commit    = "unknown"
	buildDate = "unknown"
	repoURL   = ""
)

func main() {
	if err := webi18n.Load(); err != nil {
		log.Fatalf("failed to load i18n locales: %v", err)
	}

	cfg, err := config.Load()
	if err != nil {
		log.Fatalf("failed to load config: %v", err)
	}

	db, err := persistence.NewDB(&cfg.Database)
	if err != nil {
		log.Fatalf("failed to connect database: %v", err)
	}

	repos, err := persistence.NewRepos(db)
	if err != nil {
		log.Fatalf("failed to initialize repositories: %v", err)
	}

	oidcProvider, err := oidc.NewProvider(context.Background(), &cfg.OIDC)
	if err != nil {
		log.Fatalf("failed to initialize OIDC provider: %v", err)
	}

	sessionStore, err := oidc.NewSessionStore(&cfg.OIDC.Cookie, repos.Session)
	if err != nil {
		log.Fatalf("failed to initialize session store: %v", err)
	}

	uc := app.NewUseCases(app.Repos{
		User:            repos.User,
		Dashboard:       repos.Dashboard,
		Category:        repos.Category,
		Bookmark:        repos.Bookmark,
		Application:     repos.Application,
		Setting:         repos.Setting,
		Theme:           repos.Theme,
		Session:         repos.Session,
		UserIDMigration: repos.UserIDMigration,
		IdpLink:         repos.IdpLink,
	}, validation.New())

	fiberApp := web.NewFiberApp(&cfg.App)
	web.RegisterStaticFiles(fiberApp)
	handler.RegisterAll(fiberApp, sessionStore, oidcProvider, uc, handler.BuildInfo{
		Version:   version,
		Commit:    commit,
		BuildDate: buildDate,
		RepoURL:   repoURL,
	})

	// Periodically delete expired sessions that are no longer pinned.
	go func() {
		ticker := time.NewTicker(1 * time.Hour)
		defer ticker.Stop()
		for {
			if err := uc.CleanupSessions.Handle(context.Background()); err != nil {
				log.Printf("session cleanup error: %v", err)
			}
			<-ticker.C
		}
	}()

	interruptCtx, cancel := signal.NotifyContext(
		context.Background(),
		os.Interrupt,
		syscall.SIGTERM,
	)
	defer cancel()

	addr := ":" + cfg.App.Port
	certFile := cfg.App.TLS.Cert
	keyFile := cfg.App.TLS.Key

	go func() {
		var err error
		if certFile != "" && keyFile != "" {
			log.Printf("server listening on %s (TLS)", addr)
			err = fiberApp.Listen(addr, fiber.ListenConfig{
				CertFile:    certFile,
				CertKeyFile: keyFile,
			})
		} else {
			log.Printf("server listening on %s", addr)
			err = fiberApp.Listen(addr)
		}
		if err != nil {
			log.Printf("server error: %v\n", err)
		}
	}()

	<-interruptCtx.Done()

	if err := fiberApp.Shutdown(); err != nil {
		log.Printf("server shutdown error: %v\n", err)
	}
}
