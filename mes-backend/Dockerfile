FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY target/release/mes-backend /app/mes-backend
COPY migrations /app/migrations

EXPOSE 8080

CMD ["./mes-backend"]
