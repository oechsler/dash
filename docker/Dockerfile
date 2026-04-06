FROM golang:1.25-alpine AS build

ARG VERSION=dev
ARG COMMIT=unknown
ARG BUILD_DATE=unknown

WORKDIR /src

RUN apk add --no-cache ca-certificates tzdata && update-ca-certificates

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

FROM alpine:3.22

RUN apk add --no-cache ca-certificates tzdata && \
    update-ca-certificates

RUN addgroup -S dash && \
    adduser -S -G dash dash

WORKDIR /dash

COPY --from=build /out/server /dash/server
COPY --from=build /src/delivery/web/static /dash/delivery/web/static

RUN chown -R dash:dash /dash && \
    chmod +x /dash/server

EXPOSE 8080

USER dash

CMD ["/dash/server"]
