# ------------------------------------------------------------------------------
# Use Rust image to execute a Cargo build 
# ------------------------------------------------------------------------------
FROM rust:latest as build
COPY ./ ./
RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/hello /build-out/

# ------------------------------------------------------------------------------
# Copy executable onto smallest container image
# ------------------------------------------------------------------------------
FROM debian:buster-slim
COPY --from=build /build-out/hello /usr/local/bin
ENTRYPOINT ["hello"]