# 1. This tells docker to use the Rust official image
FROM rust:latest as builder

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
RUN cargo build --release

FROM ubuntu

RUN apt update && apt install -y gettext
RUN apt install -y ncurses-term

# Copy the compiled binary from the builder stage
COPY --from=builder ./target/release/ArrTalk .
COPY --from=builder ./config.toml .

COPY config.toml config.template.toml

# Run the binary
CMD envsubst < config.template.toml > config.toml && ./ArrTalk



