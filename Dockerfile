# Stage 1: Building
FROM rust:1.74-alpine as builder

# Install musl-tools
RUN apk add --no-cache musl-dev build-base

# Add musl target
RUN rustup target add x86_64-unknown-linux-musl

# Set the working directory to 'app'
WORKDIR /usr/src/app

# Copy the source code into 'app'
COPY . .

# Build the binary with musl target
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Runtime image
FROM alpine:latest

# Create an application user
RUN addgroup -S vampire && adduser -S vampire -G vampire

USER vampire

# Set the working directory for the application to 'app'
WORKDIR /app

# Copy the compiled binary from the builder stage into 'app'
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/rust_web_api /app

# Run the app
CMD ["./rust_web_api"]
