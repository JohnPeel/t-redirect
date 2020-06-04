FROM rust:latest as builder

WORKDIR "/project/redirect"
RUN rustup default nightly && rustup update
COPY Cargo.toml Cargo.toml
COPY src src
RUN cargo fetch
ENV ROCKET_ENV=production
RUN cargo build --release

FROM rust:slim

RUN useradd rust
WORKDIR "/project/redirect"
COPY --from=builder /project/redirect/target/release/redirect ./

USER rust
EXPOSE 8000

CMD ["./redirect"]
