FROM rust:1.44.1 as build
WORKDIR /app
ADD . .
RUN rustup component add rustfmt && cargo build --release

FROM gcr.io/distroless/cc as run
COPY --from=build /app/target/release/rust-sample /rust-sample
ENTRYPOINT ["/rust-sample"]
