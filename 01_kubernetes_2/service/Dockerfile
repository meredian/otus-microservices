FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin ./service
WORKDIR ./service
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm -rf src && rm ./target/x86_64-unknown-linux-musl/release/deps/service*
ADD . ./
RUN cargo build --release

FROM alpine:latest
ARG APP=/usr/src/app
EXPOSE 3000

ENV APP_USER=appuser \
    PORT=3000 \
    RUST_LOG=api

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata dumb-init \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/service/target/x86_64-unknown-linux-musl/release/service ${APP}/service

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["/usr/bin/dumb-init", "--"]

CMD ["./service"]
