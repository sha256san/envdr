#!/usr/bin/env bash
# envdoctor (envdr) automated installer
set -euo pipefail

REPO="sha256san/envdr"
VERSION="0.1.0"
INSTALL_DIR="/usr/local/bin"

echo "🩺  envdoctor  -  Developer Environment Diagnostic Tool Installer"
echo "────────────────────────────────────────────────────────────"

ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

if [ "$OS" != "linux" ]; then
    echo "❌ Currently, the automated installer supports Linux. For other platforms, build with: cargo install --git https://github.com/${REPO}"
    exit 1
fi

if [ "$ARCH" != "x86_64" ]; then
    echo "❌ Architecture ${ARCH} is not pre-compiled. Build with: cargo install --git https://github.com/${REPO}"
    exit 1
fi

# Method 1: If Debian/Ubuntu (dpkg/apt), install .deb
if command -v dpkg >/dev/null 2>&1; then
    TMP_DEB=$(mktemp /tmp/envdoctor_XXXXXX.deb)
    DEB_URL="https://github.com/${REPO}/releases/download/v${VERSION}/envdoctor_${VERSION}_amd64.deb"
    
    echo "📥 Downloading Debian package from GitHub Releases..."
    if curl -fsSL -o "${TMP_DEB}" "${DEB_URL}" 2>/dev/null; then
        echo "📦 Installing via dpkg..."
        sudo dpkg -i "${TMP_DEB}" || sudo apt-get install -f -y
        rm -f "${TMP_DEB}"
        echo "✨ Installation complete! You can now run 'envdr' or 'envdoctor'."
        envdr --version
        exit 0
    fi
fi

# Method 2: Standalone binary installation
TMP_TAR=$(mktemp /tmp/envdr_XXXXXX.tar.gz)
TAR_URL="https://github.com/${REPO}/releases/download/v${VERSION}/envdr-v${VERSION}-linux-x86_64.tar.gz"

echo "📥 Downloading binary package from GitHub Releases..."
if curl -fsSL -o "${TMP_TAR}" "${TAR_URL}" 2>/dev/null; then
    TMP_EXTRACT=$(mktemp -d /tmp/envdr_extract_XXXXXX)
    tar -xzf "${TMP_TAR}" -C "${TMP_EXTRACT}"

    echo "📦 Copying binaries to ${INSTALL_DIR}..."
    sudo cp "${TMP_EXTRACT}"/*/envdoctor "${INSTALL_DIR}/"
    sudo cp "${TMP_EXTRACT}"/*/envdr "${INSTALL_DIR}/"
    sudo chmod 755 "${INSTALL_DIR}/envdoctor" "${INSTALL_DIR}/envdr"
    rm -rf "${TMP_TAR}" "${TMP_EXTRACT}"

    echo "✨ Installation complete! You can now run 'envdr' or 'envdoctor'."
    envdr --version
    exit 0
fi

echo "❌ Failed to download release asset from GitHub. Please visit https://github.com/${REPO}/releases to download manually."
exit 1
