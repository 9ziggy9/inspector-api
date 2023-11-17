# Use the official Rust image as a parent image
FROM rust:latest

# Create a new empty shell project
RUN USER=root cargo new --bin inspector-api
WORKDIR /inspector-api

# Copy the Cargo.toml and Cargo.lock to /inspector-api
COPY ./Cargo.toml ./Cargo.lock ./

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release
RUN rm -f ./target/release/deps/inspector-api*
RUN cargo build --release

# The final step is the command to run when the container starts
CMD ["./target/release/inspector-api"]