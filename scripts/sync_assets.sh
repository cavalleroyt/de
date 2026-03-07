#!/bin/sh
# copies the official Winux icons into the project asset folder
# adjust source path as needed when building on an actual Winux installation
# FreeBSD compatible version

SRC_DIR="/usr/share/winux/icons"
DEST_DIR="$(dirname "$0")/../assets/icons"

if [ -d "$SRC_DIR" ]; then
    mkdir -p "$DEST_DIR"
    cp -R "$SRC_DIR"/* "$DEST_DIR/"
    echo "Synced assets from $SRC_DIR to $DEST_DIR"
else
    echo "Source directory $SRC_DIR not found; falling back to bundled placeholders." >&2
    # ensure placeholder icons exist (created earlier)
    mkdir -p "$DEST_DIR"
    if [ -d "$(dirname "$0")/../assets/icons" ]; then
        cp -R "$(dirname "$0")/../assets/icons/"* "$DEST_DIR" 2>/dev/null || true
    fi
fi
