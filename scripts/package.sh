#!/usr/bin/env bash
# envdoctor multi-architecture packaging script
set -euo pipefail

VERSION="0.3.1"
DIST_DIR="dist"
TMP_DIR="target/package_tmp"

# TARGET 引数が渡されていれば使用、なければローカル環境のアーキテクチャを自動判定
TARGET="${1:-}"

if [ -z "${TARGET}" ]; then
    HOST_OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    HOST_ARCH=$(uname -m)

    case "${HOST_ARCH}" in
        x86_64|amd64)
            ARCH="x86_64"
            DEB_ARCH="amd64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            DEB_ARCH="arm64"
            ;;
        *)
            ARCH="${HOST_ARCH}"
            DEB_ARCH="${HOST_ARCH}"
            ;;
    esac

    echo "🔨 Building release binaries for host architecture (${HOST_OS}-${ARCH})..."
    cargo build --release
    BIN_DIR="target/release"
    PKG_OS="${HOST_OS}"
else
    echo "🔨 Packaging for specified target: ${TARGET}"
    if [[ "${TARGET}" == *"darwin"* ]]; then
        PKG_OS="darwin"
        if [[ "${TARGET}" == *"aarch64"* ]]; then
            ARCH="arm64"
            DEB_ARCH="arm64"
        else
            ARCH="x86_64"
            DEB_ARCH="amd64"
        fi
    elif [[ "${TARGET}" == *"aarch64"* ]]; then
        PKG_OS="linux"
        ARCH="aarch64"
        DEB_ARCH="arm64"
    else
        PKG_OS="linux"
        ARCH="x86_64"
        DEB_ARCH="amd64"
    fi

    if [ -d "target/${TARGET}/release" ]; then
        BIN_DIR="target/${TARGET}/release"
    else
        BIN_DIR="target/release"
    fi
fi

echo "📁 Preparing dist directory..."
mkdir -p "${DIST_DIR}" "${TMP_DIR}"

# 1. Binary Tarball (.tar.gz)
TAR_DIR="${TMP_DIR}/envdr-v${VERSION}-${PKG_OS}-${ARCH}"
mkdir -p "${TAR_DIR}"
cp "${BIN_DIR}/envdoctor" "${TAR_DIR}/"
cp "${BIN_DIR}/envdr" "${TAR_DIR}/"
cp README.md LICENSE CHANGELOG.md "${TAR_DIR}/"

echo "📦 Creating tar.gz archive (${DIST_DIR}/envdr-v${VERSION}-${PKG_OS}-${ARCH}.tar.gz)..."
tar -czvf "${DIST_DIR}/envdr-v${VERSION}-${PKG_OS}-${ARCH}.tar.gz" -C "${TMP_DIR}" "envdr-v${VERSION}-${PKG_OS}-${ARCH}"
cp "${DIST_DIR}/envdr-v${VERSION}-${PKG_OS}-${ARCH}.tar.gz" "${DIST_DIR}/envdoctor-v${VERSION}-${PKG_OS}-${ARCH}.tar.gz"

# 2. Debian Package (.deb) (Linux only)
if [ "${PKG_OS}" = "linux" ] && command -v dpkg-deb >/dev/null 2>&1; then
    DEB_DOC_DIR="${TMP_DIR}/deb_envdoctor"
    mkdir -p "${DEB_DOC_DIR}/DEBIAN" "${DEB_DOC_DIR}/usr/bin" "${DEB_DOC_DIR}/usr/share/doc/envdoctor"

    cat <<EOF > "${DEB_DOC_DIR}/DEBIAN/control"
Package: envdoctor
Version: ${VERSION}
Section: devel
Priority: optional
Architecture: ${DEB_ARCH}
Provides: envdr
Replaces: envdr
Maintainer: envdoctor team <sha256san@users.noreply.github.com>
Homepage: https://github.com/sha256san/envdr
Description: Automated Developer Environment Diagnostic & Health Check Tool
 envdoctor (envdr) diagnoses Python, Rust, Go, Node.js, C/C++, Docker, Git,
 GPU/CUDA/ROCm environments, identifies root causes of issues, and suggests
 actionable fixes.
EOF

    cp "${BIN_DIR}/envdoctor" "${DEB_DOC_DIR}/usr/bin/"
    cp "${BIN_DIR}/envdr" "${DEB_DOC_DIR}/usr/bin/"
    cp README.md LICENSE "${DEB_DOC_DIR}/usr/share/doc/envdoctor/"

    chmod 755 "${DEB_DOC_DIR}/usr/bin/envdoctor" "${DEB_DOC_DIR}/usr/bin/envdr"
    chmod 644 "${DEB_DOC_DIR}/DEBIAN/control" "${DEB_DOC_DIR}/usr/share/doc/envdoctor/"*

    echo "📦 Creating envdoctor_${VERSION}_${DEB_ARCH}.deb..."
    dpkg-deb --build --root-owner-group "${DEB_DOC_DIR}" "${DIST_DIR}/envdoctor_${VERSION}_${DEB_ARCH}.deb"

    # Alias .deb package
    DEB_DR_DIR="${TMP_DIR}/deb_envdr"
    mkdir -p "${DEB_DR_DIR}/DEBIAN" "${DEB_DR_DIR}/usr/bin" "${DEB_DR_DIR}/usr/share/doc/envdr"

    cat <<EOF > "${DEB_DR_DIR}/DEBIAN/control"
Package: envdr
Version: ${VERSION}
Section: devel
Priority: optional
Architecture: ${DEB_ARCH}
Provides: envdoctor
Replaces: envdoctor
Maintainer: envdoctor team <sha256san@users.noreply.github.com>
Homepage: https://github.com/sha256san/envdr
Description: Automated Developer Environment Diagnostic & Health Check Tool (envdr alias)
 envdoctor (envdr) diagnoses Python, Rust, Go, Node.js, C/C++, Docker, Git,
 GPU/CUDA/ROCm environments, identifies root causes of issues, and suggests
 actionable fixes.
EOF

    cp "${BIN_DIR}/envdoctor" "${DEB_DR_DIR}/usr/bin/"
    cp "${BIN_DIR}/envdr" "${DEB_DR_DIR}/usr/bin/"
    cp README.md LICENSE "${DEB_DR_DIR}/usr/share/doc/envdr/"

    chmod 755 "${DEB_DR_DIR}/usr/bin/envdoctor" "${DEB_DR_DIR}/usr/bin/envdr"
    chmod 644 "${DEB_DR_DIR}/DEBIAN/control" "${DEB_DR_DIR}/usr/share/doc/envdr/"*

    echo "📦 Creating envdr_${VERSION}_${DEB_ARCH}.deb..."
    dpkg-deb --build --root-owner-group "${DEB_DR_DIR}" "${DIST_DIR}/envdr_${VERSION}_${DEB_ARCH}.deb"
fi

# 3. Generate SHA256 Checksums
echo "🔒 Generating SHA256 checksums..."
cd "${DIST_DIR}"
sha256sum * > SHA256SUMS.txt
cd ..

rm -rf "${TMP_DIR}"

echo ""
echo "✨ Packaging completed successfully! Artifacts in ${DIST_DIR}/:"
ls -lh "${DIST_DIR}"
cat "${DIST_DIR}/SHA256SUMS.txt"
