FROM rustlang/rust:nightly

COPY . /app

WORKDIR /app/
RUN apt update
RUN apt install -y cmake pkg-config libssl-dev git gcc build-essential git clang libclang-dev
RUN rustup target add wasm32-unknown-unknown --toolchain nightly
RUN ./setup.sh
WORKDIR /app/node/
RUN cargo build --release
EXPOSE 9944 9933
VOLUME ["/app"]
CMD ["./target/release/feedback-node",  "--ws-external", "--rpc-external", "--dev"]