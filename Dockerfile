FROM docker.io/rust:1.76.0 as builder

WORKDIR /usr/src/app

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml ./
COPY src src
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/image_base_colors /usr/local/bin/image_base_colors

ENV APP_PORT=80
EXPOSE ${APP_PORT}

CMD ["image_base_colors"]