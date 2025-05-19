FROM rust:1.87-bullseye

WORKDIR /app

COPY . .

RUN cargo build
