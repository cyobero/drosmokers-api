FROM rust:1.57 as builder
WORKDIR /usr/src/drosmokers
ENV DATABASE_URL=postgresql://postgres:password@0.0.0.0:5444/drosmokers_db
COPY . .
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install --path .
