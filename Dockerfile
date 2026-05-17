FROM rust:1.89

WORKDIR /app

COPY Cargo.toml .
COPY src ./src

RUN cargo build --release

CMD ["bash"]