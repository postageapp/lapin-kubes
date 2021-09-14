FROM rust:1.54-buster as builder

WORKDIR /src/lapin-kubes
COPY . .

RUN cargo install --path .

FROM debian:buster

COPY --from=builder /usr/local/cargo/bin/client /usr/local/bin/client
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

WORKDIR /usr/local/bin

CMD [ "/usr/local/bin/server" ]
