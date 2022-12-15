FROM rust:1.65

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install -f wasm-bindgen-cli

WORKDIR /app
COPY . /app

EXPOSE 8080

CMD ["trunk", "serve", "--address", "0.0.0.0"]
