FROM ekidd/rust-musl-builder:nightly-2019-04-25 as builder

## Build Cache Dependency Library
RUN mkdir /tmp/app
WORKDIR /tmp/app
## Build Dependency Library with DummyVersion.toml/lock
COPY DummyVersion.toml ./Cargo.toml
COPY DummyVersion.lock ./Cargo.lock
RUN mkdir -p src/ && \
    touch src/lib.rs
RUN sudo chown -R rust:rust .
RUN cargo build --release
## Build Base Library with Cargo.toml/lock
COPY ./src/ ./src/
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
RUN sudo chown -R rust:rust .
RUN cargo build --release