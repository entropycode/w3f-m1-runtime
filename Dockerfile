FROM rustlang/rust:nightly

COPY . /app

WORKDIR /app/node

RUN apt update
RUN apt install -y cmake pkg-config libssl-dev git gcc build-essential git clang libclang-dev
RUN rustup target add wasm32-unknown-unknown --toolchain nightly
RUN cargo build --release

EXPOSE 9944 9933
VOLUME ["/app"]
CMD ["./target/release/node-template", "--dev"]