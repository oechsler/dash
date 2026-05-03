{{/*
Validation helpers to prevent misconfiguration that could cause secret drift.
*/}}
{{- define "dash.validate" -}}
{{- if .Values.postgres.enabled -}}
{{- if not .Values.dash.secrets.keys.postgresPassword -}}
{{- fail "dash.secrets.keys.postgresPassword must be set when postgres.enabled=true" -}}
{{- end -}}
{{- end -}}
{{- end -}}
