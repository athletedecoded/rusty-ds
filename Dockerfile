FROM rust:1-bullseye as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:latest

COPY --from=builder /usr/src/app/target/release/rusty-ds /usr/local/bin/rusty-ds
CMD ["rusty-ds"]
