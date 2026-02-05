#!/bin/bash
#
# ╔══════════════════════════════════════════════════════════════════╗
# ║                PROJECT ATLAS - CI/CD Setup Script                ║
# ║              System 11/300 | DevOps Infrastructure               ║
# ╚══════════════════════════════════════════════════════════════════╝
#
# This script sets up GitHub Actions CI/CD pipeline for TITAN-CLI.
# Run from the TITAN-CLI project directory.

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}"
cat << 'EOF'

 █████╗ ████████╗██╗      █████╗ ███████╗
██╔══██╗╚══██╔══╝██║     ██╔══██╗██╔════╝
███████║   ██║   ██║     ███████║███████╗
██╔══██║   ██║   ██║     ██╔══██║╚════██║
██║  ██║   ██║   ███████╗██║  ██║███████║
╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝╚══════╝

         CI/CD Pipeline Setup
           System 11/300

EOF
echo -e "${NC}"

echo -e "${CYAN}[1/5]${NC} Creating workflow directory..."
mkdir -p .github/workflows
echo -e "${GREEN}  ✓ Created .github/workflows/${NC}"

echo -e "${CYAN}[2/5]${NC} Verifying CI workflow file..."
if [ -f ".github/workflows/ci.yml" ]; then
    echo -e "${GREEN}  ✓ CI workflow exists${NC}"
else
    echo -e "${RED}  ✖ CI workflow not found${NC}"
    exit 1
fi

echo -e "${CYAN}[3/5]${NC} Running local tests..."
cargo test --verbose
echo -e "${GREEN}  ✓ All tests passed${NC}"

echo -e "${CYAN}[4/5]${NC} Checking code formatting..."
cargo fmt --all -- --check && echo -e "${GREEN}  ✓ Formatting OK${NC}" || echo -e "${YELLOW}  ⚠ Run 'cargo fmt' to fix${NC}"

echo -e "${CYAN}[5/5]${NC} Running Clippy linter..."
cargo clippy -- -D warnings && echo -e "${GREEN}  ✓ No warnings${NC}" || echo -e "${YELLOW}  ⚠ Clippy warnings found${NC}"

echo ""
echo -e "${CYAN}╔══════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${NC}           ${GREEN}✓ ATLAS CI/CD Setup Complete!${NC}                         ${CYAN}║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "Next steps:"
echo -e "  ${CYAN}1. git add .github/workflows/ci.yml${NC}"
echo -e "  ${CYAN}2. git commit -m \"ci: implement atlas pipeline with github actions\"${NC}"
echo -e "  ${CYAN}3. git push${NC}"
echo ""
echo -e "The pipeline will automatically run on GitHub!"
echo ""
