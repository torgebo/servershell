# syntax=docker/dockerfile:1

FROM rust:1.86-bookworm

RUN mkdir -p /BUILD_FOLDER

COPY . /BUILD_FOLDER/
WORKDIR /BUILD_FOLDER

RUN cargo build --release

CMD ["/BUILD_FOLDER/target/release/servershell"]


# TODO: make two stage build, with compiler and app image