FROM rust:1.56.1

WORKDIR /usr/src/petabroad-frontend
COPY . .

RUN cargo install --path .
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown

CMD trunk serve
