FROM rust:1.37.0-slim AS builder

# The Rust toolchain to use when building our image.  Set by `hooks/build`.
ARG TOOLCHAIN=stable

# The OpenSSL version to use. We parameterize this because many Rust
# projects will fail to build with 1.1.
ARG OPENSSL_VERSION=1.0.2r

RUN apt-get -y upgrade && apt-get -y update
RUN apt-get install -y openssl libssl-dev pkg-config musl-dev musl-tools curl sudo cmake build-essential

RUN mkdir app
ADD src/ app/src
ADD Cargo* app/
WORKDIR app

RUN echo "Building OpenSSL" && \
    ls /usr/include/linux && \
    sudo mkdir -p /usr/local/musl/include && \
    sudo ln -s /usr/include/linux /usr/local/musl/include/linux && \
    sudo ln -s /usr/include/x86_64-linux-gnu/asm /usr/local/musl/include/asm && \
    sudo ln -s /usr/include/asm-generic /usr/local/musl/include/asm-generic && \
    cd /tmp && \
    curl -LO "https://www.openssl.org/source/openssl-$OPENSSL_VERSION.tar.gz" && \
    tar xvzf "openssl-$OPENSSL_VERSION.tar.gz" && cd "openssl-$OPENSSL_VERSION" && \
    env CC=musl-gcc ./Configure no-shared no-zlib -fPIC --prefix=/usr/local/musl -DOPENSSL_NO_SECURE_MEMORY linux-x86_64 && \
    env C_INCLUDE_PATH=/usr/local/musl/include/ make depend && \
    env C_INCLUDE_PATH=/usr/local/musl/include/ make && \
    sudo make install && \
    sudo rm /usr/local/musl/include/linux /usr/local/musl/include/asm /usr/local/musl/include/asm-generic && \
    rm -r /tmp/*

ENV OPENSSL_DIR=/usr/local/musl/ \
    OPENSSL_INCLUDE_DIR=/usr/local/musl/include/ \
    DEP_OPENSSL_INCLUDE=/usr/local/musl/include/ \
    OPENSSL_LIB_DIR=/usr/local/musl/lib/ \
    TARGET=musl

ENV PKG_CONFIG_ALL_STATIC=true
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV OPENSSL_STATIC=1
# ENV OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu/
# ENV OPENSSL_INCLUDE_DIR=/usr/include
RUN cp /usr/include/x86_64-linux-gnu/openssl/opensslconf.h /usr/include/openssl/

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo clean && cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch
# FROM rust:1.37.0-slim
# RUN apt-get -y upgrade && apt-get -y update
# RUN apt-get install -y libssl-dev pkg-config
# CMD ["/plus9k"]
# COPY --from=builder /app/target/release/plus9k .
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/plus9k .

VOLUME ["/data"]

ENTRYPOINT ["/plus9k"]
