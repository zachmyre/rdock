# build -t rdock .
# tag rdock
# push 

# Use a Rust base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/rdock

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build the dependencies (but not the application itself)
RUN cargo fetch

# Copy the source code into the container
COPY src ./src

# Build the Rust application
RUN cargo build --release

# Use a minimal Alpine Linux image for the final container
FROM alpine:latest

# Set the working directory inside the container
WORKDIR /usr/app

# Copy the built application from the previous stage
COPY --from=builder /usr/src/rdock/target/release/rdock .

# Expose any necessary ports
# EXPOSE 8080

# Define the default command to run when the container starts
CMD ["./rdock"]


