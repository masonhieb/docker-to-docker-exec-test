FROM ubuntu:22.04

RUN apt-get update && apt-get install curl build-essential -y

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir -p /root/executor

COPY executor /root/executor

RUN cp /root/executor/accompanyments.csv /root/

WORKDIR /root/executor

RUN cargo build --release && mv /root/executor/target/release/executor /root/etemp && rm -rf /root/executor && mv /root/etemp /root/executor