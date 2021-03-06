FROM rust:1.32-stretch

WORKDIR /srv/intention_tester

ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update \
    && apt-get install -y make libssl1.0-dev \
    && apt-get clean

COPY . ./

RUN cargo build --release
ENV PATH /srv/intention_tester/target/release/:$PATH
ENTRYPOINT ["intention_tester", "-c"]
CMD ["API-url"]
