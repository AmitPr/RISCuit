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
    libslirp-dev \
    autoconf

# Build toolchain
WORKDIR /build
RUN git clone https://github.com/riscv-collab/riscv-gnu-toolchain
WORKDIR /build/riscv-gnu-toolchain
RUN ./configure --prefix=/opt/riscv
RUN make -j$(nproc 2>/dev/null || sysctl -n hw.logicalcpu) 

ENV PATH="/opt/riscv/bin:${PATH}"

# Compile riscv-tests
WORKDIR /test
RUN git clone https://github.com/riscv/riscv-tests
WORKDIR /test/riscv-tests
RUN git submodule update --init --recursive
RUN autoconf
RUN ./configure --prefix=/opt/riscv
RUN make && make install

# Copy the artifacts to /artifacts.
# This directory *should* be mounted on the host machine to actually 
# retrieve the artifacts.
ENTRYPOINT cp /test/riscv-tests/isa/* /artifacts && rm /artifacts/Makefile
