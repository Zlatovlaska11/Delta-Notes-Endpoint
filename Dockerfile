# Stage 1: Build the application
FROM rust:1.56 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo 'fn main() {}' > src/main.rs

# Build dependencies to cache them
RUN cargo build --release && rm -rf src/target/release/deps/*

# Copy the actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:buster-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/delta_notes_endpoint .

# Expose the application port (adjust if necessary)
EXPOSE 8080

# Set the entrypoint command
CMD ["./delta_notes_endpoint"]

