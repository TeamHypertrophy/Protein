# [!] Pull Rust Image From Docker
FROM rustlang/rust:nightly-slim AS build

# [!] Set Working Directory
WORKDIR /protein

# [!] Install Dependencies
RUN apt-get update -y && apt-get upgrade -y && apt-get install -y --no-install-recommends pkg-config libssl-dev libpq-dev lld clang
RUN rustup component add rustc-codegen-cranelift-preview --toolchain nightly

# [!] Build Project
COPY . .

RUN cargo build --release

# [!] Run Protein
FROM debian:bookworm-slim

WORKDIR /protein

RUN apt-get update -y && apt-get upgrade -y && apt-get install -y --no-install-recommends libssl-dev libpq-dev libssl3 ca-certificates
COPY --from=build /protein/docker.env ./.env
COPY --from=build /protein/Rocket.toml ./
COPY --from=build /protein/target/release/protein ./protein

EXPOSE 8000

CMD ["./protein"]
