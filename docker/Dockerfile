# syntax=docker/dockerfile:1

FROM golang:1.24-alpine3.20 AS build

WORKDIR /src

RUN apk add --no-cache git build-base ca-certificates && update-ca-certificates

COPY go.mod go.sum ./
RUN go mod download

COPY . .

RUN go generate ./templ && \
    go generate ./static && \
    rm -f /src/static/generate.go

RUN CGO_ENABLED=1 go build -ldflags="-s -w" -o /out/server ./cmd/server

FROM alpine:3.20

RUN apk add --no-cache ca-certificates tzdata && \
    update-ca-certificates

RUN addgroup -S dash && \
    adduser -S -G dash dash

WORKDIR /dash

COPY --from=build /out/server /dash/server
COPY --from=build /src/static /dash/static

COPY docker/entrypoint.sh /dash/entrypoint.sh

RUN chown -R dash:dash /dash && \
    chmod +x /dash/server /dash/entrypoint.sh

EXPOSE 3000

USER root
ENTRYPOINT ["/dash/entrypoint.sh"]
