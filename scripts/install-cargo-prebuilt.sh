#!/bin/bash

### Accepts in params (env vars) INSTALL_PATH, LIBC, ARCH, OS_TYPE, FORCE.
### INSTALL_PATH: Path to where cargo-prebuilt should be installed.
### LIBC: Which libc flavor to use. (gnu or musl) (Does nothing on macos)
### MINISIGN: If true, minisign will be used to verify the download. (Requires minisign to be installed)

set -euxo pipefail

# Check if cargo-prebuilt is installed already
if cargo-prebuilt --version ; then
    if [ -z ${FORCE+x}]; then
        echo "cargo-prebuilt is already installed on this system."
        echo "If you want to update it run: 'cargo-prebuilt cargo-prebuilt'."
        echo "Otherwise do 'export FORCE=true' then run this script again."
        exit 1
    fi
fi

# Start
URL="https://github.com/cargo-prebuilt/cargo-prebuilt/releases/latest/download/"

TEMP_DIR="$(mktemp -d)"
pushd "$TEMP_DIR"

: ${ARCH:="$(uname -m)"}
: ${OS_TYPE:="$(uname -s)"}
: ${LIBC:="gnu"}

: ${PUB_KEY:="RWTSqAR1Hbfu6mBFiaz4hb9I9gikhMmvKkVbyz4SJF/oxJcbbScmCqqO"}
: ${MINISIGN:="false"}

# Build target string
TARGET_STRING=""

case "$ARCH" in
    arm64|aarch64)
        TARGET_STRING+="aarch64-"
        ;;
    amd64|x86_64)
        TARGET_STRING+="x86_64-"
        ;;
    riscv64|riscv64gc)
        TARGET_STRING+="riscv64gc-"
        ;;
    s390x)
        TARGET_STRING+="s390x-"
        ;;
    armv7l|armv7)
        TARGET_STRING+="armv7-"
        ;;
    ppc64le|powerpc64le)
        TARGET_STRING+="powerpc64le-"
        ;;
    mips64|mips64el)
        TARGET_STRING+="mips64el-"
        ;;
    *)
        echo "Unsupported Arch: $ARCH" && popd && exit 1
        ;;
esac

case "$OS_TYPE" in
    Darwin)
        TARGET_STRING+="apple-darwin"
        ;;
    Linux)
        TARGET_STRING+="unknown-linux-"
        ;;
    FreeBSD)
        TARGET_STRING+="unknown-freebsd"
        ;;
    NetBSD)
        TARGET_STRING+="unknown-netbsd"
        ;;
    MINGW64*|MSYS_NT*)
        TARGET_STRING+="pc-windows-gnu"
        ;;
    *)
        echo "Unsupported OS: $OS_TYPE" && popd && exit 1
        ;;
esac

if [ "$OS_TYPE" == "Linux" ]; then
    TARGET_STRING+="$LIBC"
    case "$ARCH" in
        armv7l|armv7)
            TARGET_STRING+="eabihf"
            ;;
        mips64|mips64el)
            TARGET_STRING+="abi64"
            ;;
    esac
fi

echo "Determined target: $TARGET_STRING"

TAR="$TARGET_STRING.tar.gz"
SIG="$TAR.minisig"

# Bootstrap cargo-prebuilt
TAR_URL="$URL""$TAR"
SIG_URL="$URL""$SIG"

curl --proto '=https' --tlsv1.2 -fsSL "$TAR_URL" -o "$TAR"

if [ "$MINISIGN" == "true" ]; then
    curl --proto '=https' --tlsv1.2 -fsSL "$SIG_URL" -o "$SIG"

    if minisign --version ; then
        minisign -Vm "$TAR" -P "$PUB_KEY"
    elif rsign --version ; then
        rsign verify "$TAR" -P "$PUB_KEY"
    else
        echo "Minisign needs to be installed. (https://jedisct1.github.io/minisign)"
        echo "Or rsign2. (https://github.com/jedisct1/rsign2)"
        exit -1
    fi
fi

tar -xzvf "$TAR"

# Install cargo-prebuilt with cargo-prebuilt
ARGS=""
if [ ! -z ${INSTALL_PATH+x} ]; then
    ARGS+="--path=$INSTALL_PATH"
fi

./cargo-prebuilt $ARGS cargo-prebuilt

popd
rm -rf "$TEMP_DIR"