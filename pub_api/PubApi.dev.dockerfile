FROM rust:1.88.0

RUN cargo install cargo-watch

WORKDIR /app

COPY . .

RUN cargo fetch

CMD ["cargo", "watch", "-x", "run"]