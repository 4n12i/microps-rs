FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    iproute2 \
    iputils-ping \
    netcat-openbsd

# install task runner
# RUN cargo install just