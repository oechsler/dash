{{/*
Validation helpers to prevent misconfiguration that could cause secret drift.
*/}}
{{- define "dash.validate" -}}
{{- if .Values.postgres.enabled -}}
{{- if not .Values.dash.secrets.keys.postgresPassword -}}
{{- fail "dash.secrets.keys.postgresPassword must be set when postgres.enabled=true" -}}
{{- end -}}
{{- if and .Values.postgres.secrets.name (ne .Values.postgres.secrets.name .Values.dash.secrets.name) -}}
{{- fail (printf "postgres.secrets.name (%s) must match dash.secrets.name (%s) when postgres.enabled=true (single shared secret)." .Values.postgres.secrets.name .Values.dash.secrets.name) -}}
{{- end -}}
{{- if .Values.postgres.secrets.external.enabled -}}
{{- fail "postgres.secrets.external.enabled is not supported when postgres.enabled=true. Use dash.secrets.external.enabled and include POSTGRES_PASSWORD in the shared secret." -}}
{{- end -}}
{{- end -}}
{{- end -}}
