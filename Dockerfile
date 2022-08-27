FROM rust:1.63

COPY . .

RUN cargo build --release

EXPOSE 8000

ENV ROCKET_ADDRESS 0.0.0.0

CMD ["cargo", "run", "--release", "--bin", "squawk-server"]