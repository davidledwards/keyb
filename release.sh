#!/bin/bash

if [[ -z $(which gh) ]]; then
    echo "gh: GitHub CLI not found (see https://cli.github.com/)"
    exit 1
fi

__BIN_DIR=$(pwd)/target/release
__BIN=$__BIN_DIR/keyb

if [[ ! -e $__BIN ]]; then
    echo "$__BIN: target binary does not exist"
    exit 1
fi

# Derive version number based on what the binary reports, which itself is derived
# from the package configuration.
__VERSION=$($__BIN --version | cut -d " " -f 2)

# Create tarball and checksum artifacts.
__TAR="$__BIN_DIR/keyb-${__VERSION}-x86_64-unix.tar.gz"
tar -czf "$__TAR" -C "$__BIN_DIR keyb"
shasum -a 256 "$__TAR" > "$__TAR.sha256"

# Create actual release in GitHub
gh release create --title "$__VERSION" --generate-notes "v$__VERSION" "$__TAR"*
echo "released $__VERSION"
