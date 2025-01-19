FROM rust:latest

WORKDIR /app/

COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch
RUN apt-get update && apt-get install -y \
    jq

CMD ["cargo", "watch", "--why", "-x", "build"]