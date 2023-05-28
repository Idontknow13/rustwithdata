FROM rust:slim

WORKDIR /app

COPY . .

RUN cargo build --release

ENV DATABASE_URL "postgresql://postgres:password@rustwithdata_db/userdb"

CMD ["./target/release/rustwithdata"]