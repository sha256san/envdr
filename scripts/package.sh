#!/usr/bin/env bash
set -euo pipefail

VERSION="0.1.0"
ARCH="x86_64"
DEB_ARCH="amd64"
PKG_NAME="envdr"
DIST_DIR="dist"
TMP_DIR="target/package_tmp"

echo "🔨 Building release binaries with cargo..."
cargo build --release

echo "📁 Preparing dist directory..."
rm -rf "${DIST_DIR}" "${TMP_DIR}"
mkdir -p "${DIST_DIR}" "${TMP_DIR}"

# 1. Binary Tarball (tar.gz)
TAR_DIR="${TMP_DIR}/${PKG_NAME}-v${VERSION}-linux-${ARCH}"
mkdir -p "${TAR_DIR}"
cp target/release/envdoctor "${TAR_DIR}/"
cp target/release/envdr "${TAR_DIR}/"
cp README.md LICENSE SPEC.md CHANGELOG.md "${TAR_DIR}/"

echo "📦 Creating tar.gz archive..."
tar -czvf "${DIST_DIR}/${PKG_NAME}-v${VERSION}-linux-${ARCH}.tar.gz" -C "${TMP_DIR}" "${PKG_NAME}-v${VERSION}-linux-${ARCH}"

# 2. Debian Package (.deb)
DEB_DIR="${TMP_DIR}/deb"
mkdir -p "${DEB_DIR}/DEBIAN"
mkdir -p "${DEB_DIR}/usr/bin"
mkdir -p "${DEB_DIR}/usr/share/doc/${PKG_NAME}"

cat <<EOF > "${DEB_DIR}/DEBIAN/control"
Package: ${PKG_NAME}
Version: ${VERSION}
Section: devel
Priority: optional
Architecture: ${DEB_ARCH}
Maintainer: envdoctor team <sha256san@users.noreply.github.com>
Homepage: https://github.com/sha256san/envdr
Description: Automated Developer Environment Diagnostic & Health Check Tool
 envdoctor (envdr) diagnoses Python, Rust, Go, Node.js, C/C++, Docker, Git,
 GPU/CUDA/ROCm environments, identifies root causes of issues, and suggests
 actionable fixes.
EOF

cp target/release/envdoctor "${DEB_DIR}/usr/bin/"
cp target/release/envdr "${DEB_DIR}/usr/bin/"
cp README.md LICENSE "${DEB_DIR}/usr/share/doc/${PKG_NAME}/"

chmod 755 "${DEB_DIR}/usr/bin/envdoctor" "${DEB_DIR}/usr/bin/envdr"
chmod 644 "${DEB_DIR}/DEBIAN/control" "${DEB_DIR}/usr/share/doc/${PKG_NAME}/"*

echo "📦 Creating .deb package..."
dpkg-deb --build --root-owner-group "${DEB_DIR}" "${DIST_DIR}/${PKG_NAME}_${VERSION}_${DEB_ARCH}.deb"

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
