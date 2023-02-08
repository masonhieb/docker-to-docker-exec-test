FROM ubuntu:22.04

RUN apt-get update && apt-get install curl build-essential -y

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

COPY server /root/server

# COPY tserver.service /etc/systemd/system/

# RUN systemctl daemon-reload

WORKDIR /root/server

RUN cargo build --release && mv /root/server/target/release/server /root/etemp

WORKDIR /root

RUN rm -rf /root/server && mv /root/etemp /root/server

EXPOSE 3000

CMD [ "/root/server" ]