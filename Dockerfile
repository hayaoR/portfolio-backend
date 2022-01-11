FROM rust:1.56.0 AS builder 
WORKDIR /app

COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin portfolio 

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/portfolio portfolio 
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./portfolio"]