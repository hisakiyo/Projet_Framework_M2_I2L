FROM debian:buster AS builder

# You'll need to change `libmysqlclient-dev` to `libpq-dev` if you're using Postgres
RUN apt-get update && apt-get install -y curl default-libmysqlclient-dev build-essential

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly-2023-02-18

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

RUN cargo build --release

FROM debian:buster

RUN apt-get update && apt-get install -y default-libmysqlclient-dev

COPY --from=builder \
  /target/release/api \
  /usr/local/bin/

WORKDIR /root
CMD ROCKET_PORT=${PORT:-8000} /usr/local/bin/api
