# [!] Pull Rust Image From Docker
FROM rustlang/rust:nightly-slim AS chef

# [!] Install cargo-chef
RUN cargo install cargo-chef

# [!] Set Working Directory
WORKDIR /usr/protein

# [!] Planning Stage
FROM chef AS planner

# [!] Copy Files
COPY . .

# [!] Prepare Recipe
RUN cargo chef prepare  --recipe-path recipe.json

# [!] Building Stage
FROM chef AS build

# [!] Copy Recipe
COPY --from=planner /usr/protein/recipe.json recipe.json

# [!] Install Dependencies
RUN apt-get update -y && apt-get install -y --no-install-recommends pkg-config libssl-dev libpq-dev lld clang \
    && rustup component add rustc-codegen-cranelift-preview --toolchain nightly \
    && cargo +nightly chef cook --release --recipe-path recipe.json \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# [!] Copy Files
COPY . .

# [!] Build Project
RUN cargo build --release

# [!] Runtime Stage
FROM debian:bookworm-slim AS runtime

# [!] Set Working Directory
WORKDIR /usr/protein

# [!] Install Dependencies
RUN apt-get update -y && apt-get install -y --no-install-recommends libssl3 ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# [!] Copy Files
COPY --from=build /usr/protein/regexes.yaml ./
COPY --from=build /usr/protein/docker.env ./.env
COPY --from=build /usr/protein/Rocket.toml ./
COPY --from=build /usr/protein/target/release/protein ./protein

# [!] Expose Port
EXPOSE 8000

# [!] Run Protein
CMD ["./protein"]
