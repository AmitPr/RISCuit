FROM rustlang/rust:nightly-slim AS builder

RUN apt-get update
RUN apt-get install -y \
    which \
    sed \
    make \
    binutils \
    build-essential  \
    diffutils \
    gcc \
    g++ \
    bash \
    patch \
    gzip \
    bzip2 \
    perl \
    tar \
    cpio \
    unzip \
    rsync \
    file \
    bc \
    findutils \
    wget \
    git \
    unzip \
    cpio \
    rsync

WORKDIR /build
RUN wget http://buildroot.org/downloads/buildroot-2024.11.1.tar.xz -O buildroot.tar.xz && \
    tar -xf buildroot.tar.xz && \
    rm buildroot.tar.xz

WORKDIR /build/buildroot-2024.11.1
COPY buildroot.config /build/buildroot-2024.11.1/.config
RUN FORCE_UNSAFE_CONFIGURE=1 make -j$(nproc 2>/dev/null || sysctl -n hw.logicalcpu) sdk

ENV PATH="/build/buildroot-2024.11.1/output/host/bin:${PATH}"

# Prepare Rust
RUN rustup component add rust-src --toolchain nightly-aarch64-unknown-linux-gnu

# Copy over the target spec
WORKDIR /opt/rust/
COPY riscv32imac-unknown-linux-musl.json /opt/rust/

# Ignore missing libunwind by creating a dummy file
RUN touch /build/buildroot-2024.11.1/output/host/riscv32-riscuit-linux-musl/sysroot/usr/lib/libunwind.a
# RUN ln -s /build/buildroot-2024.11.1/output/host/lib/gcc/riscv32-buildroot-linux-musl/13.3.0/libgcc.a /build/buildroot-2024.11.1/output/host/lib/gcc/riscv32-buildroot-linux-musl/13.3.0/libunwind.a

# Build the project
# WORKDIR /code
# ENTRYPOINT cargo +nightly build --release \
#     -Z build-std=core,alloc,std,panic_abort \
#     -Z build-std-features=panic_immediate_abort,optimize_for_size \
#     --target /opt/rust/riscv32imac-unknown-linux-musl.json \
#     --config target.riscv32imac-unknown-linux-musl.linker=\"riscv32-riscuit-linux-musl-gcc\" 

# cargo +nightly build --release -Z build-std=core,alloc,std,panic_abort -Z build-std-features=panic_immediate_abort,optimize_for_size --target /opt/rust/riscv32imac-unknown-linux-musl.json --config target.riscv32imac-unknown-linux-musl.linker=\"riscv32-riscuit-linux-musl-gcc\" 
# RUSTFLAGS="-C target-feature=-crt-static"