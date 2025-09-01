# Dash

![Screenshot of Dash](./.github/assets/dash.png)

A self‑hosted, multi‑user home‑lab dashboard — your tidy command center for services, links, and apps.

Dash is a modern, multi‑user reinterpretation of the excellent [Flame](https://github.com/pawelmalak/flame) project.   
If you love [Flame's](https://github.com/pawelmalak/flame) simplicity but need user separation and a fresh Go/HTMX/templ stack, Dash is for you!

## Highlights

- Multi‑user by design: personal dashboards, settings, and themes per user
- Apps and Bookmarks: organize everything with categories (including “shelved” groups)
- Themes: custom themes via a clean settings modal
- Fast & lightweight: Go backend with [HTMX](https://htmx.org/) and [a-h/templ](https://github.com/a-h/templ) components
- SQLite by default via GORM (auto-migrations on start)
- Self‑host friendly: Docker image and Compose for quick deployment

## Screens & Structure

Key building blocks in this repo:
- Backend (Go): domain models, use cases, and endpoints under `domain/` and `endpoint/`
- Data: GORM repositories under `data/`
- UI: server-rendered templates under `templ/` using templ + HTMX interactions
- Static: embedded assets generation via `static/generate.go`

Authentication via [oauth2-proxy](https://oauth2-proxy.github.io/oauth2-proxy/):
- The Compose example includes [oauth2-proxy](https://oauth2-proxy.github.io/oauth2-proxy/) configured for an OIDC provider.
- Caddy uses forward_auth to verify sessions via `/oauth2/auth` and copies the `Authorization` header to Dash.


## Quick Start (Docker Compose)

Prerequisites: Docker + Docker Compose

Authentication overview:
- Dash expects an Authorization header (typically a Bearer access token) on incoming requests for identifying the user.
- The provided Compose example uses [oauth2-proxy](https://oauth2-proxy.github.io/oauth2-proxy/) in front of Dash to handle login and inject the Authorization header. [Caddy](https://caddyserver.com/) forwards auth via forward_auth and copies the Authorization header from [oauth2-proxy](https://oauth2-proxy.github.io/oauth2-proxy/) to Dash.
- You can run Dash without [oauth2-proxy](https://oauth2-proxy.github.io/oauth2-proxy/), but then you must ensure your reverse proxy (or your clients) attach a valid Authorization header. Without a proxy, endpoints expecting auth will treat requests as unauthenticated.

1. Clone the repo
2. Review `compose/compose.yml` and `compose/Caddyfile` (if using Caddy for TLS/edge)
3. Start the stack:

    - Basic (from repo root):
   ```sh
   docker compose -f compose/compose.yml up -d
   ```

    - With Caddy as a reverse proxy (example):
   ```sh
   docker compose -f compose/compose.yml -f compose/compose.caddy.yml up -d
   ``` 

4. Open the service URL printed by your Compose configuration:

    - Basic: http://localhost:3000
    - With Caddy: http://localhost:8080

## Building from Source

Prerequisites: Go 1.24+

1. Generate templates and static assets:
   ```sh
   go generate ./...
   ```

2. Build the server:
   ```sh
   go build -o bin/dash ./cmd/server
   ```

3. Run the server:
   ```sh
   ./bin/dash
   ```

## Project Status

⚠️ This project originated from my homelab's need for a flexible dashboard. While core features are fully functional as
I use them daily, the codebase is evolving as I continue to refine and enhance it. Consider it experimental and use in
production at your own risk.

## Contributing

Issues and PRs are welcome. Please keep changes small and focused. If adding a feature, include a short rationale and usage notes.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
