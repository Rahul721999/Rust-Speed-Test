# Stage 1: Build the Rust-Speed-Test application
FROM rust:1.70.0 as builder

WORKDIR /usr/src/rust-speed-test

# Copy only the necessary files for building the Rust-Speed-Test application
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

RUN cargo build --release

# Stage 2: Create the final image
FROM rust:1.70.0

WORKDIR /usr/bin

# Set the correct path for the executable (replace "Rust-Speed-Test" with the actual binary name)
COPY --from=builder /usr/src/rust-speed-test/target/release/Rust-Speed-Test /usr/bin/rust-speed-test

# Set the port number your application listens on (change if different from 8000)
EXPOSE 8000

# Correct the typo in the CMD instruction (use "release" instead of "relase")
CMD [ "rust-speed-test" ]
