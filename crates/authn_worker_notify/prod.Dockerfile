FROM rust:1.100-slim AS builder

ARG CARGO_BUILD_TARGET=aarch64-unknown-linux-musl

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential musl-tools perl && rm -rf /var/lib/apt/lists/*

RUN rustup target add ${CARGO_BUILD_TARGET}

WORKDIR /build/crates/authn_worker_notify

ENV CARGO_TARGET_DIR=/build/target \
    CARGO_BUILD_TARGET=${CARGO_BUILD_TARGET}

RUN mkdir -p /worker

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,from=workspace_root,source=Cargo.lock,target=/build/Cargo.lock \
    --mount=type=bind,from=workspace_root,source=Cargo.toml,target=/build/Cargo.toml \
    --mount=type=bind,from=workspace_root,source=crates/nats_messages,target=/build/crates/nats_messages \
    --mount=type=bind,from=workspace_root,source=crates/authn_service,target=/build/crates/authn_service \
    --mount=type=bind,source=templates,target=templates \
    --mount=type=cache,target=/build/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp /build/target/${CARGO_BUILD_TARGET}/release/worker /worker/worker
cp -r ./templates /worker/templates/
EOF

# ==============================================================================
FROM gcr.io/distroless/static-debian13:nonroot
WORKDIR /worker

COPY --from=builder /worker/worker /worker/worker
COPY --from=builder /worker/templates /worker/templates/

CMD ["/worker/worker"]
