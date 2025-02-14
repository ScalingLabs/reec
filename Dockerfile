FROM rust:1.79 AS buider

RUN apt-get update && apt-get install -y \ 
build-essential \
libclang-dev \
libc6 \
libssl-dev \
ca-certificates \
&& rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/reec

COPY . .

RUN cargo build --release

FROM ubuntu:24.04

WORKDIR /usr/local/bin

COPY --from=buider /usr/src/reec/target/release/reec .

EXPOSE 8545

ENTRYPOINT [ "./reec" ]



