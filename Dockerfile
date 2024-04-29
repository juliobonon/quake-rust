# syntax=docker/dockerfile:1

################################################################################
# Create a stage for building the application.

ARG RUST_VERSION=1.75.0
ARG APP_NAME=cloud-rust
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

# Build the application.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/parser
cp ./src/qgames.log /home/
EOF

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application.
FROM debian:bullseye-slim AS final

# Create a non-privileged user that the app will run under.
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/parser /bin/
COPY --from=build /home/qgames.log /home/

WORKDIR /home
ENV HOME=/home

CMD ["/bin/parser"]
