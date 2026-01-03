FROM rust:1.92-bookworm

RUN apt-get update \
    && apt-get install -y rsync \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /home/cargo

COPY ./settings/app-cargo/run.sh /home/run.sh
RUN chmod +x /home/run.sh

ENTRYPOINT ["/home/run.sh"]