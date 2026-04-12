FROM golang:1.25 AS build

ARG VERSION=dev
ARG COMMIT=unknown
ARG BUILD_DATE=unknown

WORKDIR /src

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/*

COPY go.mod go.sum ./
RUN go mod download

COPY . .

RUN go generate ./delivery/web/templ && \
    go generate ./delivery/web/static && \
    rm -f /src/delivery/web/static/generate.go

RUN CGO_ENABLED=0 go build \
    -ldflags="-s -w \
      -X main.version=${VERSION} \
      -X main.commit=${COMMIT} \
      -X main.buildDate=${BUILD_DATE}" \
    -o /out/server ./cmd/server

RUN CGO_ENABLED=0 go build -ldflags="-s -w" -o /out/healthcheck ./cmd/healthcheck

FROM gcr.io/distroless/static-debian12:nonroot

COPY --from=build /usr/share/zoneinfo /usr/share/zoneinfo
COPY --from=build /out/server /dash/server
COPY --from=build /out/healthcheck /dash/healthcheck
COPY --from=build /src/delivery/web/static /dash/delivery/web/static

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=15s --retries=3 \
  CMD ["/dash/healthcheck"]

CMD ["/dash/server"]
