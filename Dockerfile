FROM rust:latest

# Copy the files in the machine to the Docker image
COPY ./ ./

# Build the program for release
RUN cargo build --release

# Expose the port the app runs on
EXPOSE 8888

# Run the binary
CMD ["./target/release/spacesim"]