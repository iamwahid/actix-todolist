FROM rust:1.67 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:bullseye-slim AS runtime

WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
COPY entrypoint.sh .
RUN chmod +x ./entrypoint.sh

ENV MYSQL_HOST=${MYSQL_HOST:-localhost}
ENV MYSQL_PORT=${MYSQL_PORT:-3306}
ENV MYSQL_USER=${MYSQL_USER:-xxxx}
ENV MYSQL_PASSWORD=${MYSQL_PASSWORD:-xxxxx}
ENV MYSQL_DBNAME=${MYSQL_DBNAME:-todo4}

ENV APP_ENVIRONMENT production

EXPOSE 3030/tcp
ENTRYPOINT ["./entrypoint.sh"]