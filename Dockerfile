FROM rust:1-bullseye as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:latest

COPY --from=builder /usr/src/app/target/release/rust-distro-cicd /usr/local/bin/rust-distro-cicd
CMD ["rust-distro-cicd"]