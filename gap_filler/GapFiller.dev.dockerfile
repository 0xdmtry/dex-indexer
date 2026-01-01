FROM rust:1.88.0

RUN apt-get update && apt-get install -y cmake pkg-config && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch

WORKDIR /app

COPY . .

RUN cargo fetch

CMD ["cargo", "watch", "-x", "run"]