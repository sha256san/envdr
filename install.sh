#!/usr/bin/env bash
# envdoctor (envdr) automated multi-platform installer
set -euo pipefail

REPO="sha256san/envdr"
VERSION="0.2.0"
INSTALL_DIR="/usr/local/bin"

echo "🩺  envdoctor  -  Developer Environment Diagnostic Tool Installer"
echo "────────────────────────────────────────────────────────────"

RAW_ARCH=$(uname -m)
RAW_OS=$(uname -s | tr '[:upper:]' '[:lower:]')

# 1. OS 判定
case "$RAW_OS" in
    linux)
        OS="linux"
        ;;
    darwin)
        OS="darwin"
        ;;
    *)
        echo "❌ Unsupported OS: ${RAW_OS}. Please install using Cargo: cargo install --git https://github.com/${REPO}"
        exit 1
        ;;
esac

# 2. CPU アーキテクチャ判定
case "$RAW_ARCH" in
    x86_64|amd64)
        ARCH="x86_64"
        DEB_ARCH="amd64"
        DARWIN_ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        DEB_ARCH="arm64"
        DARWIN_ARCH="arm64"
        ;;
    *)
        echo "❌ Architecture ${RAW_ARCH} is not pre-compiled. Build with: cargo install --git https://github.com/${REPO}"
        exit 1
        ;;
esac

echo "ℹ️  Detected System: OS=${OS}, Arch=${ARCH}"

# 3. macOS (Apple Silicon / Intel) のインストール処理
if [ "$OS" = "darwin" ]; then
    if [ "$DARWIN_ARCH" = "arm64" ]; then
        echo "🍎 Apple Silicon (M1/M2/M3/M4) detected."
    else
        echo "🍎 Intel Mac detected."
    fi

    # Homebrew が利用可能な場合は Homebrew でのインストールを推奨・実行可能
    if command -v brew >/dev/null 2>&1; then
        echo "🍺 Homebrew detected! You can also install via: brew install sha256san/tap/envdoctor"
    fi

    TMP_TAR=$(mktemp /tmp/envdr_XXXXXX.tar.gz)
    TAR_URL="https://github.com/${REPO}/releases/download/v${VERSION}/envdr-v${VERSION}-darwin-${DARWIN_ARCH}.tar.gz"

    echo "📥 Downloading macOS release package..."
    if curl -fsSL -o "${TMP_TAR}" "${TAR_URL}" 2>/dev/null; then
        TMP_EXTRACT=$(mktemp -d /tmp/envdr_extract_XXXXXX)
        tar -xzf "${TMP_TAR}" -C "${TMP_EXTRACT}"

        echo "📦 Copying binaries to ${INSTALL_DIR}..."
        sudo cp "${TMP_EXTRACT}"/*/envdoctor "${INSTALL_DIR}/"
        sudo cp "${TMP_EXTRACT}"/*/envdr "${INSTALL_DIR}/"
        sudo chmod 755 "${INSTALL_DIR}/envdoctor" "${INSTALL_DIR}/envdr"
        rm -rf "${TMP_TAR}" "${TMP_EXTRACT}"

        echo "✨ Installation complete! You can now run 'envdr' or 'envdoctor'."
        envdr --version || true
        exit 0
    fi
fi

# 4. Linux (x86_64 / ARM64) の Debian パッケージインストール
if [ "$OS" = "linux" ] && command -v dpkg >/dev/null 2>&1; then
    TMP_DEB=$(mktemp /tmp/envdoctor_XXXXXX.deb)
    DEB_URL="https://github.com/${REPO}/releases/download/v${VERSION}/envdoctor_${VERSION}_${DEB_ARCH}.deb"
    
    echo "📥 Downloading Debian (.deb) package for ${DEB_ARCH}..."
    if curl -fsSL -o "${TMP_DEB}" "${DEB_URL}" 2>/dev/null; then
        echo "📦 Installing via dpkg..."
        sudo dpkg -i "${TMP_DEB}" || sudo apt-get install -f -y
        rm -f "${TMP_DEB}"
        echo "✨ Installation complete! You can now run 'envdr' or 'envdoctor'."
        envdr --version || true
        exit 0
    fi
fi

# 5. Linux スタンドアロンバイナリ (tar.gz) インストール
if [ "$OS" = "linux" ]; then
    TMP_TAR=$(mktemp /tmp/envdr_XXXXXX.tar.gz)
    TAR_URL="https://github.com/${REPO}/releases/download/v${VERSION}/envdr-v${VERSION}-linux-${ARCH}.tar.gz"

    echo "📥 Downloading binary package for Linux ${ARCH}..."
    if curl -fsSL -o "${TMP_TAR}" "${TAR_URL}" 2>/dev/null; then
        TMP_EXTRACT=$(mktemp -d /tmp/envdr_extract_XXXXXX)
        tar -xzf "${TMP_TAR}" -C "${TMP_EXTRACT}"

        echo "📦 Copying binaries to ${INSTALL_DIR}..."
        sudo cp "${TMP_EXTRACT}"/*/envdoctor "${INSTALL_DIR}/"
        sudo cp "${TMP_EXTRACT}"/*/envdr "${INSTALL_DIR}/"
        sudo chmod 755 "${INSTALL_DIR}/envdoctor" "${INSTALL_DIR}/envdr"
        rm -rf "${TMP_TAR}" "${TMP_EXTRACT}"

        echo "✨ Installation complete! You can now run 'envdr' or 'envdoctor'."
        envdr --version || true
        exit 0
    fi
fi

echo "❌ Failed to download pre-compiled release from GitHub."
echo "💡 You can build directly with Cargo:"
echo "   cargo install --git https://github.com/${REPO}"
exit 1
