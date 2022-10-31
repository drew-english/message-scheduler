FROM rust:1.64-bullseye AS build
WORKDIR /app
COPY src src
COPY Cargo.* sqlx-data.json .

ENV SQLX_OFFLINE=true
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo install sqlx-cli && \
    cargo build --release && \
    mv ./target/release/message-scheduler . && \
    mv /usr/local/cargo/bin/sqlx .

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=build /app/message-scheduler .
COPY --from=build /app/sqlx /usr/bin/
COPY migrations migrations
CMD ./message-scheduler
