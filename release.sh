#!/bin/bash

if [[ -z $(which gh) ]]; then
    echo "gh: GitHub CLI not found (see https://cli.github.com/)"
    exit 1
fi

function build() {
    local __TARGET="$1"
    local __BIN_DIR="$(pwd)/target/${__TARGET}/release"
    local __BIN="$__BIN_DIR/keyb"

    echo "$__TARGET: building target"
    cargo build --release --target=${__TARGET}

    if [ $? -ne 0 ]; then
        echo "$__BIN: build failure"
        exit 1
    fi

    __TAR="$__BIN_DIR/keyb-${__VERSION}-$(uname -m)-unix.tar.gz"
    tar -czf "$__TAR" -C "$__BIN_DIR" keyb
    shasum -a 256 "$__TAR" > "$__TAR.sha256"
}

echo "detecting version"
__VERSION=$(cargo run --release -- --version | cut -d " " -f 2)

__TARGETS=("aarch64-apple-darwin" "x86_64-apple-darwin")
__TARS=()
for __TARGET in "${__TARGETS[@]}"
do
    build "$__TARGET"
    __TARS+=("$__TAR" "$__TAR.sha256")
done

# Create actual release in GitHub
gh release create --title "$__VERSION" --generate-notes "v$__VERSION" ${__TARS[@]}

if [ $? -eq 0 ]; then
    echo "$__VERSION: release successful"
else
    echo "$__VERSION: release failed"
fi
