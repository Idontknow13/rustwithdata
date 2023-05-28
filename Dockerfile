FROM rust:slim

WORKDIR /app

COPY . .

# DATABASE_URL is used in the build process (by sqlx) to check for connections
ENV DATABASE_URL "postgresql://postgres:password@rustwithdata_db/userdb"

RUN cargo build --release

CMD ["./target/release/rustwithdata"]