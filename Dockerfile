FROM rust:1.80.0
COPY . .
WORKDIR /
RUN cargo build --release
CMD ["./target/release/artifactsmmorust"]