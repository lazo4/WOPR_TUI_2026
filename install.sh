#!/usr/bin/env bash
set -euo pipefail

echo "=== WOPR-2026 Installer ==="

# ── Detect OS ──
OS="$(uname -s)"
ARCH="$(uname -m)"
echo "Detected: $OS / $ARCH"

# ── Install Rust + Cargo if missing ──
if ! command -v cargo &>/dev/null; then
    echo "Rust not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "Rust installed: $(rustc --version)"
else
    echo "Rust found: $(rustc --version)"
fi

# ── macOS: ensure Xcode CLI tools (provides linker + system headers) ──
if [ "$OS" = "Darwin" ]; then
    if ! xcode-select -p &>/dev/null; then
        echo "Installing Xcode Command Line Tools..."
        xcode-select --install
        echo "Waiting for Xcode CLI tools install to complete..."
        until xcode-select -p &>/dev/null; do sleep 5; done
    fi
fi

# ── Linux: ensure cc + pkg-config + OpenSSL dev headers (reqwest needs them) ──
if [ "$OS" = "Linux" ]; then
    MISSING=""
    command -v cc    &>/dev/null || MISSING="$MISSING build-essential"
    command -v pkg-config &>/dev/null || MISSING="$MISSING pkg-config"
    if [ ! -f /usr/include/openssl/ssl.h ] && [ ! -f /usr/include/x86_64-linux-gnu/openssl/ssl.h ]; then
        MISSING="$MISSING libssl-dev"
    fi
    if [ -n "$MISSING" ]; then
        echo "Installing system deps:$MISSING"
        if command -v apt-get &>/dev/null; then
            sudo apt-get update -qq && sudo apt-get install -y $MISSING
        elif command -v dnf &>/dev/null; then
            # Fedora/RHEL names differ
            MISSING="${MISSING//build-essential/gcc}"
            MISSING="${MISSING//libssl-dev/openssl-devel}"
            sudo dnf install -y $MISSING
        elif command -v pacman &>/dev/null; then
            MISSING="${MISSING//build-essential/base-devel}"
            MISSING="${MISSING//libssl-dev/openssl}"
            sudo pacman -Sy --noconfirm $MISSING
        else
            echo "ERROR: Can't auto-install:$MISSING — install them manually and re-run."
            exit 1
        fi
    fi
fi

# ── Build + install ──
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
echo "Building wopr-2026 (release)..."
cargo install --path "$SCRIPT_DIR" --force

# ── Verify ──
if command -v wopr-2026 &>/dev/null; then
    echo ""
    echo "✓ Installed successfully!"
    echo "  Run:  wopr-2026"
    echo "  Path: $(which wopr-2026)"
else
    CARGO_BIN="${CARGO_HOME:-$HOME/.cargo}/bin"
    echo ""
    echo "Built successfully, but wopr-2026 not on PATH."
    echo "Add this to your shell profile:"
    echo "  export PATH=\"$CARGO_BIN:\$PATH\""
fi
