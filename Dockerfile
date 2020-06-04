FROM rust:alpine as builder

WORKDIR "/project/redirect"
RUN rustup default nightly && rustup update
COPY Cargo.toml Cargo.toml
RUN cargo fetch
COPY src src
ENV ROCKET_ENV=production
RUN cargo build --release

FROM alpine

RUN useradd rust
WORKDIR "/project/redirect"
COPY --from=builder /project/redirect/target/release/ ./

USER rust
EXPOSE 8000

CMD ["./cfclicker.exe"]
