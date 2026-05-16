#!/bin/bash
set -e

echo "=== MinimalMark - Fedora Build Script ==="
echo ""

echo "Installing build dependencies..."
sudo dnf install -y \
    rust \
    cargo \
    gtk4-devel \
    glib2-devel \
    webkit2gtk4.1-devel \
    libadwaita-devel \
    gtksourceview5-devel

echo ""
echo "Building MinimalMark..."
cargo build --release

echo ""
echo "Installing to /usr/local..."
sudo install -Dm755 target/release/minimalmark /usr/local/bin/minimalmark
sudo install -Dm644 data/io.github.minimalmark.desktop /usr/share/applications/io.github.minimalmark.desktop
sudo install -Dm644 data/io.github.minimalmark.metainfo.xml /usr/share/metainfo/io.github.minimalmark.metainfo.xml
sudo install -Dm644 data/style.css /usr/share/minimalmark/style.css
sudo install -Dm644 data/icons/hicolor/scalable/apps/io.github.minimalmark.svg /usr/share/icons/hicolor/scalable/apps/io.github.minimalmark.svg

echo ""
echo "Done! Run 'minimalmark' to start."
