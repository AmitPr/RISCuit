FROM rustlang/rust:nightly-slim AS builder

# TODO: Variables instead of hardcoding

# Dependencies
RUN apt-get update
RUN apt-get install -y \
    gcc-riscv64-linux-gnu \
    binutils-riscv64-linux-gnu \
    autoconf \
    automake \
    autotools-dev \
    curl \
    python3 \
    python3-pip \
    python3-tomli \
    libmpc-dev \
    libmpfr-dev \
    libgmp-dev \
    gawk \
    build-essential \
    bison \
    flex \
    texinfo \
    gperf \
    libtool \
    patchutils \
    bc \
    zlib1g-dev \
    libexpat-dev \
    ninja-build \
    git \
    cmake \
    libglib2.0-dev \
    libslirp-dev

# Build riscv32gc-unknown-linux-gnu toolchain
WORKDIR /build
RUN git clone https://github.com/riscv-collab/riscv-gnu-toolchain
WORKDIR /build/riscv-gnu-toolchain
RUN ./configure --prefix=/opt/riscv --with-arch=rv32imac --with-cmodel=medany --with-abi=ilp32 --enable-multilib
RUN make -j$(nproc 2>/dev/null || sysctl -n hw.logicalcpu) linux

ENV PATH="/opt/riscv/bin:${PATH}"

# Prepare Rust
RUN rustup component add rust-src --toolchain nightly-aarch64-unknown-linux-gnu

# Copy over the target spec (TODO: proper fs location)
WORKDIR /opt/rust/
COPY riscv32imac-unknown-linux-gnu.json /opt/rust/

# Build the project
WORKDIR /code
ENTRYPOINT cargo +nightly build --release \
    -Z build-std=core,alloc,std,panic_abort \
    -Z build-std-features=optimize_for_size \
    --target /opt/rust/riscv32imac-unknown-linux-gnu.json \
    --config target.riscv32imac-unknown-linux-gnu.linker=\"riscv32-unknown-linux-gnu-gcc\" \
