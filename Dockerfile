FROM rust:1.40.0-stretch as build

ARG SCCACHE_ENDPOINT
ARG SCCACHE_BUCKET
ARG AWS_SECRET_ACCESS_KEY
ARG AWS_ACCESS_KEY_ID

WORKDIR /build
COPY Cargo.toml /build/

RUN apt-get update &&\
    apt-get install -y cmake &&\
    apt-get clean && rm -rf /var/lib/apt/lists/*

RUN mkdir src &&\
    echo "pub fn main() {}" > src/main.rs &&\
    cargo build --release 

RUN rm -rf /build/src
RUN rm -rf /build/target/release/.fingerprint/reader-*

# Now copy over the source and compile our app
COPY src/ /build/src/
RUN cargo build --release

# ==================================================

FROM debian:stretch-20190910 as service

RUN apt-get update &&\
    apt-get install -y curl &&\
    apt-get clean && rm -rf /var/lib/apt/lists/* &&\
    curl -fsSL https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh > /usr/local/bin/wait-for-it &&\
    chmod +x /usr/local/bin/wait-for-it

WORKDIR /app
COPY log4rs.yml /app/
COPY --from=build /build/target/release/reader /app

# CMD ["./reader", "--brokers" ]

CMD /usr/local/bin/wait-for-it --timeout=60 kafka:9092 &&\
    /usr/local/bin/wait-for-it --timeout=60 materialized:6875 &&\
    # /app/reader --brokers kafka --group-id reader --topics mysql.simple.purchase
    # /app/reader --brokers kafka --group-id reader --topics mysql.simple.region
    /app/reader --brokers kafka:9092 --group-id reader --topics topic1
