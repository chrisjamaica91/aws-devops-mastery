# Stage 1: Build stage (compile Rust code)
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /app

# Install musl-tools for static linking
RUN apt-get update && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    rm -rf /var/lib/apt/lists/*

# Copy all source files
COPY Cargo.toml ./
COPY src ./src

# Build the application (statically linked binary)
RUN cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/rust-processor

# Stage 2: Runtime stage (distroless for maximum security)
FROM gcr.io/distroless/static-debian12:nonroot

# Copy only the binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-processor /app/rust-processor

# Expose port 3000
EXPOSE 3000

# Set environment variables
ENV RUST_LOG=info

# Run the application
ENTRYPOINT ["/app/rust-processor"]