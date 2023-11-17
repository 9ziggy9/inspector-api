# Use the official Rust image as a parent image
FROM rust:latest

# Create a working directory
WORKDIR /usr/src/inspector-api

# Copy the Cargo.toml and Cargo.lock to /usr/src/inspector-api
COPY ./Cargo.toml ./Cargo.lock ./

# Cache dependencies
# This step creates a dummy main file to build and cache dependencies
RUN mkdir src && \
    echo "fn main() {println!(\"If you see this, the build cache was used\")}" > src/main.rs && \
    cargo build --release

# Now copy your actual source tree
COPY ./src ./src
COPY ./.env ./

# Rebuild for release, this time with your actual source files
RUN touch src/main.rs && \
    cargo build --release

# The final step is the command to run when the container starts
CMD ["./target/release/inspector-api"]