FROM rust:1.78.0-slim-bookworm as base

RUN apt update -y
RUN apt install -y libssl-dev pkg-config

FROM base as build

RUN USER=root cargo new --bin fbxcli
WORKDIR /fbxcli

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/fbx_cli*
RUN cargo build --release

FROM scratch

COPY --from=build /fbxcli/target/release/fbx_cli .
CMD ["./fbx_cli"]