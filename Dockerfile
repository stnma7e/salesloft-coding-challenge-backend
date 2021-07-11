FROM rust

RUN USER=root cargo new --bin app
WORKDIR /app

COPY ./Cargo.toml Cargo.toml
COPY ./Cargo.lock Cargo.lock

RUN cargo build --release
RUN rm -rf src
RUN rm -rf target/release/deps/app*

COPY . ./
RUN cargo build --release

CMD cargo run --release
