#!/bin/bash
# Build Rust binaries on vsprod and deploy them
# Use this if cargo is not available locally

set -e

HOST="vsprod"
CGI_BIN_DIR="/usr/lib/cgi-bin"
BUILD_DIR="/tmp/robocyril-build-$$"

echo "=== Building Rust binaries on vsprod ==="
echo ""

# Check connection
if ! ssh "$HOST" "echo 'Connected to vsprod'" 2>/dev/null; then
    echo "ERROR: Cannot connect to $HOST"
    exit 1
fi

echo "✓ Connected to vsprod"
echo ""

# Create build directory on vsprod
echo "Creating build directory on vsprod..."
ssh "$HOST" "mkdir -p $BUILD_DIR"

# Copy source files to vsprod
echo "Copying source files..."
scp -r robocyril "$HOST:$BUILD_DIR/"

# Build on vsprod
echo "Building binaries on vsprod (this may take a few minutes)..."
ssh "$HOST" "source \$HOME/.cargo/env && cd $BUILD_DIR/robocyril && cargo build --release --bin blog-feed --bin blog-projects --bin blog-post --bin blog-update"

if [ $? -ne 0 ]; then
    echo "ERROR: Build failed on vsprod"
    echo "Cleaning up..."
    ssh "$HOST" "rm -rf $BUILD_DIR"
    exit 1
fi

echo "✓ Build successful"
echo ""

# Deploy binaries
echo "Deploying binaries to $CGI_BIN_DIR..."
ssh "$HOST" "sudo cp $BUILD_DIR/robocyril/target/release/blog-feed $CGI_BIN_DIR/ && \
             sudo cp $BUILD_DIR/robocyril/target/release/blog-projects $CGI_BIN_DIR/ && \
             sudo cp $BUILD_DIR/robocyril/target/release/blog-post $CGI_BIN_DIR/ && \
             sudo cp $BUILD_DIR/robocyril/target/release/blog-update $CGI_BIN_DIR/ && \
             sudo chmod +x $CGI_BIN_DIR/blog-feed $CGI_BIN_DIR/blog-projects $CGI_BIN_DIR/blog-post $CGI_BIN_DIR/blog-update && \
             sudo chown www-data:www-data $CGI_BIN_DIR/blog-feed $CGI_BIN_DIR/blog-projects $CGI_BIN_DIR/blog-post $CGI_BIN_DIR/blog-update"

echo "✓ Binaries deployed"
echo ""

# Cleanup
echo "Cleaning up build directory..."
ssh "$HOST" "rm -rf $BUILD_DIR"

echo ""
echo "=== Build Complete ==="
echo "Deployed binaries:"
echo "  - $CGI_BIN_DIR/blog-feed"
echo "  - $CGI_BIN_DIR/blog-projects"
echo "  - $CGI_BIN_DIR/blog-post"
echo "  - $CGI_BIN_DIR/blog-update"
echo ""
echo "✓ All done!"
