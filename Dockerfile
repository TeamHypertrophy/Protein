# [!] Pull Rust Image From Docker
FROM rust:1.82.0

# [-] Copy Files From Local to Container
COPY ./ ./

# [-] Build Binary For Release 
RUN cargo build --release

# [!!] Run Binary
CMD ["./target/release/protein"]