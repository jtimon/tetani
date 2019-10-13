FROM ubuntu:18.10

RUN apt-get -yqq update \
  && apt-get install -qfy \
    build-essential \
    curl \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /wd

# Install cargo for rust builds
RUN cd /root \
    && curl -s -L -O https://static.rust-lang.org/rustup.sh \
    && bash ./rustup.sh -y --verbose
ENV PATH="/root/.cargo/bin:${PATH}"

COPY Cargo.toml /wd/Cargo.toml
COPY src /wd/src
RUN cd /wd && \
    cargo build --verbose --all && \
    cargo test --verbose --all
