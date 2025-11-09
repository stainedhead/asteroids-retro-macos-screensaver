#!/bin/bash
set -e  # Exit on error

echo "================================================"
echo "Asteroids Retro - Local Deployment Script"
echo "================================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Configuration
APP_NAME="AsteroidsRetro"
BUNDLE_ID="com.stainedhead.asteroids-retro"
VERSION="1.0.0"
SCREENSAVER_DIR="$HOME/Library/Screen Savers"

echo -e "${BLUE}Step 1: Running tests...${NC}"
cargo test --verbose
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Tests passed${NC}"
else
    echo -e "${RED}✗ Tests failed${NC}"
    exit 1
fi
echo ""

echo -e "${BLUE}Step 2: Building release binary...${NC}"
cargo build --release --target aarch64-apple-darwin
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi
echo ""

echo -e "${BLUE}Step 3: Creating app bundle structure...${NC}"
rm -rf "${APP_NAME}.app"
mkdir -p "${APP_NAME}.app/Contents/MacOS"
mkdir -p "${APP_NAME}.app/Contents/Resources"
echo -e "${GREEN}✓ Bundle structure created${NC}"
echo ""

echo -e "${BLUE}Step 4: Copying binary to app bundle...${NC}"
cp "target/aarch64-apple-darwin/release/asteroids_screensaver" "${APP_NAME}.app/Contents/MacOS/${APP_NAME}"
chmod +x "${APP_NAME}.app/Contents/MacOS/${APP_NAME}"
echo -e "${GREEN}✓ Binary copied${NC}"
echo ""

echo -e "${BLUE}Step 5: Creating Info.plist...${NC}"
cat > "${APP_NAME}.app/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>${BUNDLE_ID}</string>
    <key>CFBundleName</key>
    <string>Asteroids Retro</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>12.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2024 stainedhead. MIT License.</string>
</dict>
</plist>
EOF
echo -e "${GREEN}✓ Info.plist created${NC}"
echo ""

echo -e "${BLUE}Step 6: Deploying to screensaver directory...${NC}"
# Create screensaver directory if it doesn't exist
mkdir -p "${SCREENSAVER_DIR}"

# Remove existing installation if present
if [ -d "${SCREENSAVER_DIR}/${APP_NAME}.app" ]; then
    echo "Removing existing installation..."
    rm -rf "${SCREENSAVER_DIR}/${APP_NAME}.app"
fi

# Copy to screensaver directory
cp -R "${APP_NAME}.app" "${SCREENSAVER_DIR}/"
echo -e "${GREEN}✓ Deployed to ${SCREENSAVER_DIR}/${NC}"
echo ""

echo "================================================"
echo -e "${GREEN}Deployment Complete!${NC}"
echo "================================================"
echo ""
echo "The screensaver has been installed to:"
echo "  ${SCREENSAVER_DIR}/${APP_NAME}.app"
echo ""
echo "To use it:"
echo "  1. Open System Settings/Preferences"
echo "  2. Go to 'Screen Saver' (or 'Desktop & Screen Saver')"
echo "  3. Select 'Asteroids Retro' from the list"
echo "  4. Click 'Preview' to test it"
echo ""
echo "Or run directly from Terminal:"
echo "  \"${SCREENSAVER_DIR}/${APP_NAME}.app/Contents/MacOS/${APP_NAME}\""
echo ""
