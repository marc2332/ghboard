FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin ghboard
RUN ls

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt update
RUN apt install ca-certificates -y
WORKDIR /app
COPY . . 
COPY public/ /usr/local/bin
COPY --from=builder /app/target/release/ghboard /usr/local/bin
ENTRYPOINT ["/usr/local/bin/ghboard"]
