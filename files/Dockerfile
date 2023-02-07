FROM rust:latest
WORKDIR /usr/src/files
COPY . /usr/src/files
EXPOSE 8080
RUN cargo build
CMD cargo run