FROM rust:1-slim-bullseye AS chef

WORKDIR /app

RUN apt-get update \
    && apt-get install -y \
    git \
    grep \
    cloc \
    pkg-config \
    openssl \
    libssl-dev \
    && cargo install cargo-watch --locked \
    && cargo install cargo-chef --locked \
    && cargo install sqlx-cli --no-default-features --features native-tls,postgres \
    && rustup component add clippy \
    && rustup component add rustfmt \
    && rm -rf /var/lib/apt/lists/*

# This stage is common for both dev and prod.
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# This is the profile that we run for development.
FROM chef AS dev
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --recipe-path recipe.json
# Build application
COPY . .
CMD cargo update
CMD cargo watch --clear --quiet --exec clippy --exec run

#CMD cargo watch --clear --quiet -x clippy -x run
