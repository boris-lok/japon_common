FROM rust:1.60 as builder

WORKDIR /usr/src/customer_services

COPY ./customer_services .
COPY ./japon_common ./common

RUN rm -rf build.rs

RUN cargo build --release

WORKDIR /usr/src/product_services

COPY ./product_services .
COPY ./japon_common ./common

RUN rm -rf build.rs

RUN cargo build --release

WORKDIR /usr/src/web_api_gateway

COPY ./web_api_gateway .
COPY ./japon_common ./common

RUN rm -rf build.rs

RUN cargo build --release

FROM debian:latest
WORKDIR /opt

RUN mkdir logs

COPY ./customer_services/env ./env
COPY ./entrypoint.sh .

COPY --from=builder /usr/src/customer_services/target/release/customer_services .
COPY --from=builder /usr/src/product_services/target/release/product_services .
COPY --from=builder /usr/src/web_api_gateway/target/release/web_api_gateway .

EXPOSE 3030 

CMD ["./entrypoint.sh"]
