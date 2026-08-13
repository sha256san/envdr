#!/usr/bin/env bash
set -euo pipefail

VERSION="0.1.0"
ARCH="x86_64"
DEB_ARCH="amd64"
DIST_DIR="dist"
TMP_DIR="target/package_tmp"

echo "🔨 Building release binaries with cargo..."
cargo build --release

echo "📁 Preparing dist directory..."
rm -rf "${DIST_DIR}" "${TMP_DIR}"
mkdir -p "${DIST_DIR}" "${TMP_DIR}"

# 1. Binary Tarball (tar.gz)
TAR_DIR="${TMP_DIR}/envdr-v${VERSION}-linux-${ARCH}"
mkdir -p "${TAR_DIR}"
cp target/release/envdoctor "${TAR_DIR}/"
cp target/release/envdr "${TAR_DIR}/"
cp README.md LICENSE CHANGELOG.md "${TAR_DIR}/"

echo "📦 Creating tar.gz archive..."
tar -czvf "${DIST_DIR}/envdr-v${VERSION}-linux-${ARCH}.tar.gz" -C "${TMP_DIR}" "envdr-v${VERSION}-linux-${ARCH}"
cp "${DIST_DIR}/envdr-v${VERSION}-linux-${ARCH}.tar.gz" "${DIST_DIR}/envdoctor-v${VERSION}-linux-${ARCH}.tar.gz"

# 2. Debian Package (.deb) for envdoctor (Package: envdoctor, Provides: envdr)
DEB_DOC_DIR="${TMP_DIR}/deb_envdoctor"
mkdir -p "${DEB_DOC_DIR}/DEBIAN"
mkdir -p "${DEB_DOC_DIR}/usr/bin"
mkdir -p "${DEB_DOC_DIR}/usr/share/doc/envdoctor"

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

cp target/release/envdoctor "${DEB_DOC_DIR}/usr/bin/"
cp target/release/envdr "${DEB_DOC_DIR}/usr/bin/"
cp README.md LICENSE "${DEB_DOC_DIR}/usr/share/doc/envdoctor/"

chmod 755 "${DEB_DOC_DIR}/usr/bin/envdoctor" "${DEB_DOC_DIR}/usr/bin/envdr"
chmod 644 "${DEB_DOC_DIR}/DEBIAN/control" "${DEB_DOC_DIR}/usr/share/doc/envdoctor/"*

echo "📦 Creating envdoctor .deb package..."
dpkg-deb --build --root-owner-group "${DEB_DOC_DIR}" "${DIST_DIR}/envdoctor_${VERSION}_${DEB_ARCH}.deb"

# 3. Debian Package (.deb) for envdr (Package: envdr, Provides: envdoctor)
DEB_DR_DIR="${TMP_DIR}/deb_envdr"
mkdir -p "${DEB_DR_DIR}/DEBIAN"
mkdir -p "${DEB_DR_DIR}/usr/bin"
mkdir -p "${DEB_DR_DIR}/usr/share/doc/envdr"

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

cp target/release/envdoctor "${DEB_DR_DIR}/usr/bin/"
cp target/release/envdr "${DEB_DR_DIR}/usr/bin/"
cp README.md LICENSE "${DEB_DR_DIR}/usr/share/doc/envdr/"

chmod 755 "${DEB_DR_DIR}/usr/bin/envdoctor" "${DEB_DR_DIR}/usr/bin/envdr"
chmod 644 "${DEB_DR_DIR}/DEBIAN/control" "${DEB_DR_DIR}/usr/share/doc/envdr/"*

echo "📦 Creating envdr .deb package..."
dpkg-deb --build --root-owner-group "${DEB_DR_DIR}" "${DIST_DIR}/envdr_${VERSION}_${DEB_ARCH}.deb"

# 4. Generate SHA256 Checksums
echo "🔒 Generating SHA256 checksums..."
cd "${DIST_DIR}"
sha256sum * > SHA256SUMS.txt
cd ..

rm -rf "${TMP_DIR}"

echo ""
echo "✨ Packaging completed successfully! Artifacts in ${DIST_DIR}/:"
ls -lh "${DIST_DIR}"
cat "${DIST_DIR}/SHA256SUMS.txt"
