# MathHook Multi-Platform Builder
# Builds Rust/Python/Node packages for all platforms via cross-compilation
#
# Targets:
#   Linux:   x86_64-gnu, x86_64-musl, aarch64-gnu, aarch64-musl
#   macOS:   x86_64, aarch64 (via zig)
#   Windows: x86_64-msvc (via xwin)

FROM rust:slim-bookworm AS builder

# Prevent interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# System dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Cross-compilation toolchains
    gcc-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    gcc-x86-64-linux-gnu \
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
    shellcheck \
    jq \
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
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Zig (for macOS cross-compilation)
# pip installs to /usr/local/lib/python*/dist-packages/ziglang/, symlink to PATH
RUN pip3 install --break-system-packages ziglang \
    && ln -sf $(python3 -c "import ziglang; print(ziglang.__file__.replace('__init__.py', 'zig'))") /usr/local/bin/zig

# Optional: macOS SDK for cross-compilation (~1GB)
ARG WITH_MACOS_SDK=true
RUN if [ "$WITH_MACOS_SDK" = "true" ]; then \
    curl -L https://github.com/joseluisq/macosx-sdks/releases/download/14.5/MacOSX14.5.sdk.tar.xz \
    | tar -xJ -C /opt \
    && ln -s /opt/MacOSX14.5.sdk /opt/MacOSX.sdk; \
    fi

# Set macOS SDK environment (only used if SDK exists)
ENV SDKROOT=/opt/MacOSX.sdk
ENV MACOSX_DEPLOYMENT_TARGET=11.0


# Install cargo tools
RUN cargo install cargo-zigbuild cargo-audit --locked

# Optional: xwin for Windows MSVC SDK (~5GB)
ARG WITH_WINDOWS_SDK=true
RUN if [ "$WITH_WINDOWS_SDK" = "true" ]; then \
    cargo install xwin --locked \
    && xwin --accept-license splat --output /opt/xwin; \
    fi

# Configure xwin environment for Windows builds
ENV XWIN_ARCH=x86_64
ENV XWIN_VARIANT=desktop
ENV XWIN_SDK_PATH=/opt/xwin

# Install maturin (Python wheel builder)
RUN pip3 install --break-system-packages maturin twine

# Ensure pyo3 detects Python correctly for cross-compilation
ENV PYO3_PYTHON=/usr/bin/python3

# Install napi-rs CLI and TypeScript (Node.js native addon builder)
RUN npm install -g @napi-rs/cli typescript

# Add Rust targets and components
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
    x86_64-pc-windows-msvc \
    && rustup component add clippy rustfmt

# Configure cargo for cross-compilation
RUN mkdir -p /root/.cargo
COPY docker/cargo-config.toml /root/.cargo/config.toml

# Set environment for Windows cross-compilation
ENV CC_x86_64_pc_windows_msvc=clang-cl
ENV CXX_x86_64_pc_windows_msvc=clang-cl
ENV AR_x86_64_pc_windows_msvc=llvm-lib
ENV INCLUDE="/opt/xwin/crt/include;/opt/xwin/sdk/include/ucrt;/opt/xwin/sdk/include/um;/opt/xwin/sdk/include/shared"
ENV LIB="/opt/xwin/crt/lib/x86_64;/opt/xwin/sdk/lib/um/x86_64;/opt/xwin/sdk/lib/ucrt/x86_64"
ENV CARGO_TARGET_X86_64_APPLE_DARWIN_RUSTFLAGS="-C link-arg=-F/opt/MacOSX.sdk/System/Library/Frameworks -C link-arg=-L/opt/MacOSX.sdk/usr/lib -C link-arg=-isysroot -C link-arg=/opt/MacOSX.sdk"
ENV CARGO_TARGET_AARCH64_APPLE_DARWIN_RUSTFLAGS="-C link-arg=-F/opt/MacOSX.sdk/System/Library/Frameworks -C link-arg=-L/opt/MacOSX.sdk/usr/lib -C link-arg=-isysroot -C link-arg=/opt/MacOSX.sdk"

WORKDIR /build

# Default command
CMD ["bash"]
