FROM rust:1.56.1

WORKDIR /usr/src/petabroad-backend
COPY . .

RUN cargo install --path .

CMD ["petabroad-backend"]
