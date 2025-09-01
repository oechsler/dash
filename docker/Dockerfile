# syntax=docker/dockerfile:1

# Build stage
FROM golang:1.24-alpine3.20 AS build

WORKDIR /src

# Install build dependencies
RUN apk add --no-cache git build-base ca-certificates && update-ca-certificates

# Install templ generator
RUN go install github.com/a-h/templ/cmd/templ@latest

# Cache modules
COPY go.mod go.sum ./
RUN go mod download

# Copy the rest of the source
COPY . .

# Generate templ components and static assets
# 1) Run templ generate via go:generate in templ/generate.go
# 2) Run static asset generator via go:generate in static/generate.go
# 3) Remove static/generate.go to prevent it from being served in the container
RUN go generate ./templ && go generate ./static && rm -f /src/static/generate.go

# Build the server
RUN CGO_ENABLED=1 go build -ldflags="-s -w" -o /out/server ./cmd/server

# Runtime stage
FROM alpine:3.20

# Install CA certificates for HTTPS calls (if the app makes outbound HTTPS requests)
RUN apk add --no-cache ca-certificates tzdata && update-ca-certificates

# Create non-root user and group
RUN addgroup -S dash && adduser -S -G dash dash

WORKDIR /dash

# Copy server binary and static assets
COPY --from=build /out/server /dash/server
COPY --from=build /src/static /dash/static

# Add entrypoint script (path is relative to build context root)
COPY docker/entrypoint.sh /dash/entrypoint.sh

# Adjust ownership and permissions
RUN chown -R dash:dash /dash && chmod +x /dash/server /dash/entrypoint.sh

# Expose a conventional port (adjust if your server uses another)
EXPOSE 3000

# Run as root to fix volume permissions at runtime; entrypoint will drop to dash user
USER root
ENTRYPOINT ["/dash/entrypoint.sh"]
