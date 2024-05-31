FROM ubuntu:20.04
WORKDIR /app
RUN apt update
RUN apt install -y llvm-10-dev libclang-common-10-dev clang curl build-essential zlib1g-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . "$HOME/.cargo/env" && rustup override set nightly

# DOCKER BUILD:         docker build -t rust-llvm .
# RUN ECHO:             docker run -it --rm -v $PWD:/app -w /app rust-llvm bash
# Run Oko compiler:     cargo  run
