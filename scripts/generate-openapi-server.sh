#!/usr/bin/env bash
set -euo pipefail

readonly expected_version="7.26.0"
readonly repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly input_spec="${repository_root}/spec/openapi.yaml"
readonly output_dir="${repository_root}/generated/openapi"
generator="${OPENAPI_GENERATOR_CLI:-openapi-generator-cli}"

if ! command -v "${generator}" >/dev/null 2>&1; then
	echo "OpenAPI Generator CLI not found: ${generator}" >&2
	echo "Install OpenAPI Generator ${expected_version} and retry." >&2
	exit 1
fi

actual_version="$("${generator}" version)"
if [[ "${actual_version}" != "${expected_version}" ]]; then
	echo "Expected OpenAPI Generator ${expected_version}, found ${actual_version}." >&2
	exit 1
fi

"${generator}" validate --input-spec "${input_spec}"
"${generator}" generate \
	--input-spec "${input_spec}" \
	--generator-name rust-axum \
	--output "${output_dir}" \
	--additional-properties=packageName=dash_api,packageVersion=0.1.0,generateAliasAsModel=true

cargo fmt --all
