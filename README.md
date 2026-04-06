# Dash


![Screenshot of Dash](./.github/assets/dash.png)

A self-hosted, multi-user home-lab dashboard — inspired by [Flame](https://github.com/pawelmalak/flame), rebuilt for modern homelabs.

Sign in with your existing identity provider, get a personal dashboard with bookmarks and themes, and share application links with friends and family.

> [!WARNING]
> This project emerged from my personal homelab needs for a flexible multi-user dashboard. While the core features are production-ready and actively used in my daily workflow, the codebase is under active development as I continue to add features and refinements. Early adopters should note this experimental status and evaluate carefully before deploying in critical production environments.

## Highlights

- Multi-user with OAuth2/OIDC
- Per-user dashboards, bookmarks, and themes
- Shared application links with group-based visibility
- PostgreSQL backed, Docker image for amd64 and arm64

## Quick Start

```bashgit
cp docker/compose/dash.env.example docker/compose/dash.env
cp docker/compose/postgres.env.example docker/compose/postgres.env
# fill in both files

docker compose -f docker/compose/compose.yml up -d
```

## Images

Images are published to two registries on every push to `main` (`:main`) and on every version tag (`:v*`, `:latest`):

- `git.at.oechsler.it/samuel/dash`
- `ghcr.io/oechsler/dash`

## Contributing

Contributions and pull requests are welcome!

## License

MIT — see [LICENSE](./LICENSE).
