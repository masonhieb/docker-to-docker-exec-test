FROM ubuntu:22.04

RUN apt-get update && apt-get install curl ca-certificates gnupg lsb-release build-essential -y

# Install docker in this docker container

RUN mkdir -m 0755 -p /etc/apt/keyrings && curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg

RUN echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
    $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null

RUN apt-get update && apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y

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