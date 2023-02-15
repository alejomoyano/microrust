FROM rust:latest
WORKDIR /usr/src/auth
COPY . .
EXPOSE 8081
RUN cargo build
CMD cargo run