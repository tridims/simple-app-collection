# build
FROM rust:1.69-buster as builder

WORKDIR /app

# Accept the build argument
ARG DATABASE_URL

# Make sure to use the ARG in ENV
ENV DATABASE_URL=$DATABASE_URL

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/simple-note-api .

CMD ["./simple-note-api"]