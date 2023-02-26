# 1. This tells docker to use the Rust official image
FROM rustlang/rust:nightly

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
RUN cargo build --release

RUN cargo install diesel_cli --no-default-features --features mysql
RUN apt-get update && apt-get install -y netcat

CMD bash -c "while ! nc -z db 3306; do sleep 1; done; diesel migration run --database-url=mysql://root:Csec8m5VQH@db/projet_db && ./target/release/api"
