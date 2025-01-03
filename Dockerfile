# [!] Pull Rust Image From Docker
FROM rustlang/rust:nightly-slim AS build

# [!] Set Working Directory
WORKDIR /protein

# [!] Copy Project Files
COPY . .

# [!] Install Dependencies
RUN apt-get update -y && apt-get upgrade -y 
RUN apt-get install -y pkg-config libssl-dev libpq-dev
RUN apt-get install -y lld clang
RUN rustup component add rustc-codegen-cranelift-preview --toolchain nightly
RUN cargo build --release

# [!] Run Protein
FROM debian:bookworm-slim

WORKDIR /protein

RUN apt-get update -y && apt-get upgrade -y && apt-get install -y pkg-config libssl-dev libpq-dev
COPY --from=build /protein/target/release/protein ./protein

EXPOSE 8000

CMD ["./protein"]