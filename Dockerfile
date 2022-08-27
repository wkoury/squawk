FROM rust:1.63

COPY . .

RUN cargo build --release

EXPOSE 8000

CMD ["./target/release/squawk-server"]