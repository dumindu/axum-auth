FROM rust:1.100-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /build/crates/authn_worker_notify

ENV CARGO_TARGET_DIR=/build/target

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
cp /build/target/release/worker /worker/worker
cp -r ./templates /worker/templates/
EOF

WORKDIR /worker
CMD ["/worker/worker"]
