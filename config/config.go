package config

type Config struct {
	App      AppConfig      `yaml:"app"`
	Database DatabaseConfig `yaml:"database"`
	OIDC     OIDCConfig     `yaml:"oidc"`
}

type AppConfig struct {
	Env  string    `yaml:"env"  env:"APP_ENV"  env-default:"prod"`
	Port string    `yaml:"port" env:"APP_PORT" env-default:"8080"`
	TLS  TLSConfig `yaml:"tls"`
}

type TLSConfig struct {
	Cert string `yaml:"cert" env:"APP_TLS_CERT_FILE"`
	Key  string `yaml:"key"  env:"APP_TLS_KEY_FILE"`
}

type DatabaseConfig struct {
	URL string `yaml:"url" env:"DATABASE_URL" env-required:"true"`
}

type OIDCConfig struct {
	Issuer        string           `yaml:"issuer"          env:"OIDC_ISSUER"          env-required:"true"`
	ClientID      string           `yaml:"client_id"       env:"OIDC_CLIENT_ID"       env-required:"true"`
	ClientSecret  string           `yaml:"client_secret"   env:"OIDC_CLIENT_SECRET"   env-required:"true"`
	RedirectURL   string           `yaml:"redirect_url"    env:"OIDC_REDIRECT_URL"    env-required:"true"`
	EndSessionURL string           `yaml:"end_session_url" env:"OIDC_END_SESSION_URL"`
	Scopes        string           `yaml:"scopes"          env:"OIDC_SCOPES"          env-default:"openid profile email groups"`
	AdminGroup    string           `yaml:"admin_group"     env:"OIDC_ADMIN_GROUP"     env-default:"admin"`
	ProfileURL    string           `yaml:"profile_url"     env:"OIDC_PROFILE_URL"`
	Cookie        OIDCCookieConfig `yaml:"cookie"`
}

type OIDCCookieConfig struct {
	HashKey  string `yaml:"hash_key"  env:"OIDC_COOKIE_HASH_KEY"  env-required:"true"`
	BlockKey string `yaml:"block_key" env:"OIDC_COOKIE_BLOCK_KEY" env-required:"true"`
	Name     string `yaml:"name"      env:"OIDC_COOKIE_NAME"      env-default:"dash-session"`
	Domain   string `yaml:"domain"    env:"OIDC_COOKIE_DOMAIN"`
	Secure   bool   `yaml:"secure"    env:"OIDC_COOKIE_SECURE"    env-default:"true"`
	MaxAge   int    `yaml:"max_age"   env:"OIDC_COOKIE_MAX_AGE"   env-default:"0"`
}
