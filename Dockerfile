FROM rust:1.93-alpine

WORKDIR /server

COPY ./cce2e_server/ .

RUN cargo check &&\
    cargo build -r

EXPOSE 7007

CMD [ "/" ]
