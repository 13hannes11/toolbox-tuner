FROM quay.io/podman/stable

ENV RUST_VERSION=1.61.0
ENV HOME=/home/root

RUN dnf install gtk4-devel gcc libadwaita-devel -y
RUN dnf install toolbox -y

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN . $HOME/.cargo/env
ENV PATH=/home/root/.cargo/bin:$PATH
RUN rustup install ${RUST_VERSION}

WORKDIR /mnt

CMD cargo test

