ARG RUST_VERSION=1.81.0

FROM docker.io/rust:${RUST_VERSION} as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

RUN rm -rf target/release/app

COPY . .

RUN cargo build --release

CMD ["tail", "-f", "/dev/null"]