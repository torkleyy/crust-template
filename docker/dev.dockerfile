FROM rust:1.56

RUN apt-get update -y && \
    apt-get install --no-install-recommends -y \
      build-essential git clang cmake libstdc++-10-dev libssl-dev libxxhash-dev zlib1g-dev \
      libsoup2.4-dev libpango1.0-dev libgtk-3-dev patchelf librsvg2-dev curl wget \
      libwebkit2gtk-4.0-dev && \
    mkdir -p /setup && \
    cd /setup && \
    # compile & install mold for fast link times
    git clone https://github.com/rui314/mold.git && \
    cd mold && \
    git checkout v0.9.6 && \
    make -j$(nproc) && \
    make install && \
    # hack to make everything use mold over lld \
    # our .cargo/config tells cargo to use lld (so it uses mold bc of this hack)
    ln -s /usr/bin/mold /usr/bin/ld.lld

ARG ADDITIONAL_PKGS=""

RUN apt-get install -y ${ADDITIONAL_PKGS}

ARG DEFAULT_UID=1000

RUN useradd -ms /bin/bash --uid ${DEFAULT_UID} user

USER user

ENV NODE_VERSION=16.13.0
ENV NVM_DIR=/home/user/.nvm
ENV PATH="${NVM_DIR}/versions/node/v${NODE_VERSION}/bin/:${PATH}"

# install nodejs
RUN mkdir -p ${NVM_DIR} && \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash && \
    . "$NVM_DIR/nvm.sh" && \
    nvm install ${NODE_VERSION} && \
    nvm use v${NODE_VERSION} && \
    nvm alias default v${NODE_VERSION} && \
    node --version && \
    npm --version && \
    # install yarn
    npm install --global yarn && \
    yarn --version

VOLUME ["/usr/local/cargo/registry"]
VOLUME ["/app"]
WORKDIR /app

ENTRYPOINT ["cargo"]
CMD ["check"]
