# MathHook Multi-Platform Builder
# Builds Rust/Python/Node packages for all platforms via cross-compilation
#
# Targets:
#   Linux:   x86_64-gnu, x86_64-musl, aarch64-gnu, aarch64-musl
#   macOS:   x86_64, aarch64 (via zig)
#   Windows: x86_64-msvc (via xwin)

FROM rust:1.83-slim-bookworm AS builder

# Prevent interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# System dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Cross-compilation toolchains
    gcc-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    gcc-x86-64-linux-gnu \
    musl-tools \
    musl-dev \
    # Build essentials
    pkg-config \
    libssl-dev \
    build-essential \
    # Utilities
    curl \
    wget \
    unzip \
    git \
    ca-certificates \
    # Python
    python3 \
    python3-pip \
    python3-venv \
    # For clang-cl (Windows builds)
    clang \
    lld \
    llvm \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 20 LTS
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Zig 0.13.0 (for macOS cross-compilation)
ARG ZIG_VERSION=0.13.0
RUN curl -L "https://ziglang.org/download/${ZIG_VERSION}/zig-linux-x86_64-${ZIG_VERSION}.tar.xz" \
    | tar -xJ -C /usr/local \
    && ln -s /usr/local/zig-linux-x86_64-${ZIG_VERSION}/zig /usr/local/bin/zig

# Install cargo-zigbuild for zig-based cross-compilation
RUN cargo install cargo-zigbuild --locked

# Install xwin for Windows MSVC SDK (takes ~5GB, cached in layer)
RUN cargo install xwin --locked \
    && xwin --accept-license splat --output /opt/xwin

# Configure xwin environment for Windows builds
ENV XWIN_ARCH=x86_64
ENV XWIN_VARIANT=desktop
ENV XWIN_SDK_PATH=/opt/xwin

# Install maturin (Python wheel builder) - pinned for reproducibility
RUN pip3 install --break-system-packages maturin==1.7.4 twine==5.1.1

# Install napi-rs CLI (Node.js native addon builder) - pinned for reproducibility
RUN npm install -g @napi-rs/cli@3.0.0-alpha.63

# Add Rust targets
RUN rustup target add \
    # Linux
    x86_64-unknown-linux-gnu \
    x86_64-unknown-linux-musl \
    aarch64-unknown-linux-gnu \
    aarch64-unknown-linux-musl \
    # macOS (cross-compile via zig)
    x86_64-apple-darwin \
    aarch64-apple-darwin \
    # Windows (cross-compile via xwin)
    x86_64-pc-windows-msvc

# Configure cargo for cross-compilation
RUN mkdir -p /root/.cargo
COPY docker/cargo-config.toml /root/.cargo/config.toml

# Set environment for Windows cross-compilation
ENV CC_x86_64_pc_windows_msvc=clang-cl
ENV CXX_x86_64_pc_windows_msvc=clang-cl
ENV AR_x86_64_pc_windows_msvc=llvm-lib
ENV INCLUDE="/opt/xwin/crt/include;/opt/xwin/sdk/include/ucrt;/opt/xwin/sdk/include/um;/opt/xwin/sdk/include/shared"
ENV LIB="/opt/xwin/crt/lib/x86_64;/opt/xwin/sdk/lib/um/x86_64;/opt/xwin/sdk/lib/ucrt/x86_64"

WORKDIR /build

# Default command
CMD ["bash"]
