# Use the official Rust image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/velovite-back

# Copy the current directory contents into the container
COPY . .

# Build the Rust project
RUN cargo build --release

# Run the compiled binary
CMD ["sh", "-c", "./target/release/velovite-back $BACKEND_PARAM"]