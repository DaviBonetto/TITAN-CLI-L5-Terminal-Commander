#!/bin/bash
#
# ╔══════════════════════════════════════════════════════════════════╗
# ║                  TITAN-CLI Setup Script                          ║
# ║              System 10/300 | L5 Interface Layer                  ║
# ╚══════════════════════════════════════════════════════════════════╝
#
# This script sets up the TITAN-CLI development environment.
# Run from home directory: cd ~ && ./setup_titan_cli.sh

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Banner
echo -e "${CYAN}"
cat << 'EOF'

████████╗██╗████████╗ █████╗ ███╗   ██╗       ██████╗██╗     ██╗
╚══██╔══╝██║╚══██╔══╝██╔══██╗████╗  ██║      ██╔════╝██║     ██║
   ██║   ██║   ██║   ███████║██╔██╗ ██║█████╗██║     ██║     ██║
   ██║   ██║   ██║   ██╔══██║██║╚██╗██║╚════╝██║     ██║     ██║
   ██║   ██║   ██║   ██║  ██║██║ ╚████║      ╚██████╗███████╗██║
   ╚═╝   ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═══╝       ╚═════╝╚══════╝╚═╝

                    ⚡ SETUP SCRIPT ⚡

EOF
echo -e "${NC}"

# Configuration
REPO_NAME="TITAN-CLI-L5-Terminal-Commander"
REPO_URL="https://github.com/DaviBonetto/TITAN-CLI-L5-Terminal-Commander.git"

echo -e "${CYAN}[1/6]${NC} Checking prerequisites..."

# Check for Rust
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}  ✓ Rust installed: ${RUST_VERSION}${NC}"
else
    echo -e "${YELLOW}  ⚠ Rust not found. Installing...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}  ✓ Rust installed${NC}"
fi

echo -e "${CYAN}[2/6]${NC} Cloning repository..."

# Clone or enter repo
if [ -d "$REPO_NAME" ]; then
    echo -e "${YELLOW}  ⚠ Directory exists. Entering...${NC}"
    cd "$REPO_NAME"
    git pull origin main 2>/dev/null || true
else
    git clone "$REPO_URL" 2>/dev/null || true
    cd "$REPO_NAME"
fi

echo -e "${GREEN}  ✓ Repository ready${NC}"

echo -e "${CYAN}[3/6]${NC} Initializing Rust project..."

# Check if Cargo.toml exists
if [ ! -f "Cargo.toml" ]; then
    echo -e "${YELLOW}  ⚠ No Cargo.toml found. Initializing...${NC}"
    cargo init --bin
fi

echo -e "${GREEN}  ✓ Rust project initialized${NC}"

echo -e "${CYAN}[4/6]${NC} Installing dependencies..."

# Update cargo
cargo update 2>/dev/null || true
echo -e "${GREEN}  ✓ Dependencies resolved${NC}"

echo -e "${CYAN}[5/6]${NC} Building release binary..."

# Build
cargo build --release

BINARY_PATH="./target/release/titan"
if [ -f "$BINARY_PATH" ]; then
    BINARY_SIZE=$(ls -lh "$BINARY_PATH" | awk '{print $5}')
    echo -e "${GREEN}  ✓ Binary built: titan (${BINARY_SIZE})${NC}"
else
    # Try Windows binary name
    BINARY_PATH="./target/release/titan.exe"
    if [ -f "$BINARY_PATH" ]; then
        BINARY_SIZE=$(ls -lh "$BINARY_PATH" | awk '{print $5}')
        echo -e "${GREEN}  ✓ Binary built: titan.exe (${BINARY_SIZE})${NC}"
    fi
fi

echo -e "${CYAN}[6/6]${NC} Verifying installation..."

# Test the binary
if [ -f "./target/release/titan" ]; then
    ./target/release/titan --version && echo -e "${GREEN}  ✓ TITAN-CLI operational${NC}"
elif [ -f "./target/release/titan.exe" ]; then
    ./target/release/titan.exe --version && echo -e "${GREEN}  ✓ TITAN-CLI operational${NC}"
else
    cargo run -- --version && echo -e "${GREEN}  ✓ TITAN-CLI operational${NC}"
fi

# Summary
echo ""
echo -e "${CYAN}╔══════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${NC}               ${GREEN}✓ TITAN-CLI Setup Complete!${NC}                       ${CYAN}║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "Try these commands:"
echo -e "  ${CYAN}cargo run -- status${NC}              # Check service status"
echo -e "  ${CYAN}cargo run -- ask \"Hello VORTEX\"${NC}  # Query the AI"
echo -e "  ${CYAN}cargo run -- vision --stream${NC}     # Connect to OPTICUS"
echo -e "  ${CYAN}cargo run -- version${NC}             # Show version info"
echo ""
echo -e "Or use the release binary:"
echo -e "  ${CYAN}./target/release/titan status${NC}"
echo ""
