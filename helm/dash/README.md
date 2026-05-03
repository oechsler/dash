# dash Helm chart

This chart deploys Dash (and optionally an internal PostgreSQL).

## Secrets (required)

By default, the chart expects pre-existing Kubernetes Secrets (no secret values in `values.yaml`).

### Shared secret

Create a Secret named `dash.secrets.name` containing:
- `OIDC_CLIENT_SECRET`
- `OIDC_COOKIE_HASH_KEY`
- `OIDC_COOKIE_BLOCK_KEY`

#### Internal Postgres (postgres.enabled=true)

Add:
- `POSTGRES_PASSWORD`

The chart will generate `DATABASE_URL` automatically from `POSTGRES_PASSWORD` plus the configured
Postgres service/user/database values, so you don't need to store `DATABASE_URL` in a Secret.

#### External Postgres (postgres.enabled=false)

Add:
- `DATABASE_URL`

## Optional: render blueprints

To render placeholder Secret manifests (for documentation/bootstrapping), enable:
- `dash.secrets.blueprint.enabled=true`

These manifests contain placeholder values (`REPLACE_ME`) and must be replaced before use.

## Optional: External Secrets Operator

If you enable `dash.secrets.external.enabled=true`, this chart renders `ExternalSecret` resources
(requires External Secrets Operator CRDs).

## External Postgres

Set `postgres.enabled=false` and provide `DATABASE_URL` via the Dash secret.
