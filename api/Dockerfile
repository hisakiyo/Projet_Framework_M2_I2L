FROM rustlang/rust:nightly-slim
 
WORKDIR /app
COPY . .
 
RUN cargo install --path .

RUN cargo install cargo-watch 