# dash Helm chart

This chart deploys Dash (and optionally an internal PostgreSQL).

## Secrets (required)

By default, the chart expects pre-existing Kubernetes Secrets (no secret values in `values.yaml`).

### Dash secret

Create a Secret named `dash.secrets.name` containing:
- `DATABASE_URL`
- `OIDC_CLIENT_SECRET`
- `OIDC_COOKIE_HASH_KEY`
- `OIDC_COOKIE_BLOCK_KEY`

### Postgres secret (only if postgres.enabled=true)

Create a Secret named `postgres.secrets.name` containing:
- `POSTGRES_PASSWORD`

## Optional: render blueprints

To render placeholder Secret manifests (for documentation/bootstrapping), enable:
- `dash.secrets.blueprint.enabled=true`
- `postgres.secrets.blueprint.enabled=true`

These manifests contain placeholder values (`REPLACE_ME`) and must be replaced before use.

## Optional: External Secrets Operator

If you enable `dash.secrets.external.enabled=true` and/or `postgres.secrets.external.enabled=true`,
this chart renders `ExternalSecret` resources (requires External Secrets Operator CRDs).

## External Postgres

Set `postgres.enabled=false` and provide `DATABASE_URL` via the Dash secret.
