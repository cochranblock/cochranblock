FROM rust:1.83-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --features approuter

FROM debian:bookworm-slim
RUN apt-get update -qq && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/cochranblock /usr/local/bin/
EXPOSE 8081
ENV PORT=8081
ENV BIND=0.0.0.0
CMD ["cochranblock"]
