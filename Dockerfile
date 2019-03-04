FROM rust:1.32-stretch as builder

WORKDIR /srv/intention_tester

ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update \
    && apt-get install -y make libssl1.0-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY . ./

RUN cargo build --release
RUN cargo run
