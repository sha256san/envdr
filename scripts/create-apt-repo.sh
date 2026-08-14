#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="docs/apt"
VERSION="0.3.2"
PKG_DEB="dist/envdoctor_${VERSION}_amd64.deb"

if [ ! -f "${PKG_DEB}" ]; then
    echo "Building packages first..."
    ./scripts/package.sh
fi

echo "📁 Setting up APT repository in ${REPO_DIR}..."
rm -rf "${REPO_DIR}"
mkdir -p "${REPO_DIR}/pool/main/e/envdoctor"
mkdir -p "${REPO_DIR}/dists/stable/main/binary-amd64"

cp "dist/envdoctor_${VERSION}_amd64.deb" "${REPO_DIR}/pool/main/e/envdoctor/"
cp "dist/envdr_${VERSION}_amd64.deb" "${REPO_DIR}/pool/main/e/envdoctor/"

cd "${REPO_DIR}"

PKG_FILE="pool/main/e/envdoctor/envdoctor_${VERSION}_amd64.deb"
SIZE=$(stat -c%s "${PKG_FILE}")
SHA256=$(sha256sum "${PKG_FILE}" | awk '{print $1}')
MD5=$(md5sum "${PKG_FILE}" | awk '{print $1}')
SHA1=$(sha1sum "${PKG_FILE}" | awk '{print $1}')

# Generate Packages file
cat <<EOF > dists/stable/main/binary-amd64/Packages
Package: envdoctor
Version: ${VERSION}
Section: devel
Priority: optional
Architecture: amd64
Maintainer: envdoctor team <sha256san@users.noreply.github.com>
Homepage: https://github.com/sha256san/envdr
Provides: envdr
Description: Automated Developer Environment Diagnostic & Health Check Tool
 envdoctor (envdr) diagnoses Python, Rust, Go, Node.js, C/C++, Docker, Git,
 GPU/CUDA/ROCm environments, identifies root causes of issues, and suggests
 actionable fixes.
Filename: ${PKG_FILE}
Size: ${SIZE}
MD5sum: ${MD5}
SHA1: ${SHA1}
SHA256: ${SHA256}

Package: envdr
Version: ${VERSION}
Section: devel
Priority: optional
Architecture: amd64
Maintainer: envdoctor team <sha256san@users.noreply.github.com>
Homepage: https://github.com/sha256san/envdr
Provides: envdoctor
Description: Automated Developer Environment Diagnostic & Health Check Tool (envdr alias)
Filename: pool/main/e/envdoctor/envdr_${VERSION}_amd64.deb
Size: $(stat -c%s "pool/main/e/envdoctor/envdr_${VERSION}_amd64.deb")
MD5sum: $(md5sum "pool/main/e/envdoctor/envdr_${VERSION}_amd64.deb" | awk '{print $1}')
SHA1: $(sha1sum "pool/main/e/envdoctor/envdr_${VERSION}_amd64.deb" | awk '{print $1}')
SHA256: $(sha256sum "pool/main/e/envdoctor/envdr_${VERSION}_amd64.deb" | awk '{print $1}')
EOF

gzip -9c dists/stable/main/binary-amd64/Packages > dists/stable/main/binary-amd64/Packages.gz

PKG_GZ_SIZE=$(stat -c%s dists/stable/main/binary-amd64/Packages.gz)
PKG_GZ_SHA256=$(sha256sum dists/stable/main/binary-amd64/Packages.gz | awk '{print $1}')
PKG_RAW_SIZE=$(stat -c%s dists/stable/main/binary-amd64/Packages)
PKG_RAW_SHA256=$(sha256sum dists/stable/main/binary-amd64/Packages | awk '{print $1}')

cat <<EOF > dists/stable/Release
Origin: envdoctor
Label: envdoctor
Suite: stable
Codename: stable
Architectures: amd64
Components: main
Description: envdoctor APT repository
Date: $(date -Ru)
SHA256:
 ${PKG_RAW_SHA256} ${PKG_RAW_SIZE} main/binary-amd64/Packages
 ${PKG_GZ_SHA256} ${PKG_GZ_SIZE} main/binary-amd64/Packages.gz
EOF

cp dists/stable/Release dists/stable/main/binary-amd64/Release

echo "✨ APT repository generated successfully in ${REPO_DIR}/!"
