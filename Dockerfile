FROM rust:1.59.0-alpine3.15 AS builder
# dependencies only
RUN apk add musl-dev
RUN mkdir src
RUN echo 'fn main() {}' > src/main.rs
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo build --release
# our code
COPY . .
RUN cargo build --release

FROM alpine:3.15 AS runner
# name matches with Cargo.toml
COPY --from=builder target/release/try-axum .
# port matches with main.rs
EXPOSE 3000
ENTRYPOINT ["./try-axum"]
