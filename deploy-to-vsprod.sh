#!/bin/bash
# Deployment script for vsprod:/var/www/devblog and /usr/lib/cgi-bin/
# Deploys the new RSS 2.0 feed and project page features

set -e

HOST="vsprod"
CGI_BIN_DIR="/usr/lib/cgi-bin"
DEVBLOG_WWW="/var/www/devblog"

echo "=== Robocyril Deployment to vsprod ==="
echo ""

# Check if we can reach the host
if ! ssh "$HOST" "echo 'Connected to vsprod'" 2>/dev/null; then
    echo "ERROR: Cannot connect to $HOST"
    echo "Please ensure SSH is configured for vsprod"
    exit 1
fi

echo "✓ Connected to vsprod"
echo ""

# Step 1: Build Rust binaries locally (if cargo is available)
echo "=== Step 1: Rust Binaries ==="
if command -v cargo &> /dev/null; then
    echo "Building Rust binaries locally..."
    cd robocyril
    cargo build --release --bin blog-feed --bin blog-projects
    cd ..
    echo "✓ Rust binaries built"

    # Copy binaries to vsprod
    echo "Copying blog-feed to vsprod..."
    scp robocyril/target/release/blog-feed "$HOST:/tmp/"
    ssh "$HOST" "sudo mv /tmp/blog-feed $CGI_BIN_DIR/ && sudo chmod +x $CGI_BIN_DIR/blog-feed && sudo chown www-data:www-data $CGI_BIN_DIR/blog-feed"
    echo "✓ blog-feed deployed"

    echo "Copying blog-projects to vsprod..."
    scp robocyril/target/release/blog-projects "$HOST:/tmp/"
    ssh "$HOST" "sudo mv /tmp/blog-projects $CGI_BIN_DIR/ && sudo chmod +x $CGI_BIN_DIR/blog-projects && sudo chown www-data:www-data $CGI_BIN_DIR/blog-projects"
    echo "✓ blog-projects deployed"
else
    echo "WARNING: cargo not found, skipping Rust binary build"
    echo "You'll need to build and deploy blog-feed and blog-projects manually"
fi
echo ""

# Step 2: Deploy CGI wrapper scripts
echo "=== Step 2: CGI Wrapper Scripts ==="

# Deploy cyril wrapper scripts
echo "Deploying blog wrapper scripts..."
scp cgi-wrappers/blog-feed.cgi "$HOST:/tmp/"
ssh "$HOST" "sudo mv /tmp/blog-feed.cgi $CGI_BIN_DIR/ && sudo chmod +x $CGI_BIN_DIR/blog-feed.cgi && sudo chown www-data:www-data $CGI_BIN_DIR/blog-feed.cgi"
echo "✓ blog-feed.cgi deployed"

# Deploy devblog wrapper scripts
echo "Deploying devblog wrapper scripts..."
for script in devblog-feed.cgi devblog-projects.cgi; do
    scp "devblog-wrappers/$script" "$HOST:/tmp/"
    ssh "$HOST" "sudo mv /tmp/$script $CGI_BIN_DIR/ && sudo chmod +x $CGI_BIN_DIR/$script && sudo chown www-data:www-data $CGI_BIN_DIR/$script"
    echo "✓ $script deployed"
done
echo ""

# Step 3: Build and deploy devblog frontend
echo "=== Step 3: Devblog Frontend ==="
if command -v npm &> /dev/null; then
    echo "Building devblog frontend..."
    cd devblog-frontend
    npm install
    npm run build
    cd ..
    echo "✓ Frontend built"

    echo "Deploying devblog frontend to $DEVBLOG_WWW..."
    # Create temp directory with proper permissions, copy files, then move
    ssh "$HOST" "mkdir -p /tmp/devblog-deploy-\$\$"
    TEMP_DIR=$(ssh "$HOST" "echo /tmp/devblog-deploy-\$\$")
    scp -r devblog-frontend/dist/* "$HOST:$TEMP_DIR/"
    ssh "$HOST" "sudo mkdir -p $DEVBLOG_WWW && sudo cp -r $TEMP_DIR/* $DEVBLOG_WWW/ && sudo chown -R www-data:www-data $DEVBLOG_WWW && rm -rf $TEMP_DIR"
    echo "✓ Frontend deployed"
else
    echo "WARNING: npm not found, skipping frontend build"
    echo "You'll need to build and deploy the frontend manually"
fi
echo ""

# Step 4: Update lighttpd configuration (if needed)
echo "=== Step 4: Configuration Check ==="
echo "Checking if lighttpd config needs updating..."
ssh "$HOST" "if [ -f /etc/lighttpd/conf-available/99-cyril.conf ]; then echo 'Config exists'; else echo 'Config missing - needs manual setup'; fi"

echo ""
echo "=== Deployment Summary ==="
echo "Deployed components:"
echo "  - RSS 2.0 feed: blog-feed binary + wrapper scripts"
echo "  - Projects API: blog-projects binary + devblog-projects.cgi wrapper"
echo "  - Devblog frontend: built and deployed to $DEVBLOG_WWW"
echo ""
echo "RSS feed endpoints:"
echo "  - Cyril blog: https://steponnopets.net/cyril/feed.xml"
echo "  - Devblog: https://steponnopets.net/devblog/feed.xml"
echo ""
echo "Projects API endpoints:"
echo "  - Devblog: https://steponnopets.net/devblog/api/projects"
echo ""
echo "Next steps:"
echo "  1. Test RSS feeds: curl https://steponnopets.net/cyril/feed.xml"
echo "  2. Test projects API: curl https://steponnopets.net/devblog/api/projects"
echo "  3. Test devblog UI: https://steponnopets.net/devblog/"
echo "  4. If lighttpd config is missing, copy lighttpd-cyril.conf to vsprod"
echo ""
echo "✓ Deployment complete!"
