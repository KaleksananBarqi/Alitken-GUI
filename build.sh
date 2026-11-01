#!/usr/bin/env bash
# ==============================================================================
# Alitken Media Converter - Linux Master Developer & Packaging Tool
# ==============================================================================
# Equivalent to build.ps1 for Linux environments (Fedora/RPM & AppImage).
# ==============================================================================

set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}${BOLD}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}${BOLD}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}${BOLD}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}${BOLD}[ERROR]${NC} $1"
}

get_version() {
    if [ -f "package.json" ]; then
        node -p "require('./package.json').version" 2>/dev/null || echo "0.9.2"
    else
        echo "0.9.2"
    fi
}

check_dependencies() {
    log_info "Checking prerequisites..."
    local missing=0

    if ! command -v node >/dev/null 2>&1; then
        log_error "Node.js is not installed or not in PATH."
        missing=1
    fi
    if ! command -v npm >/dev/null 2>&1; then
        log_error "NPM is not installed or not in PATH."
        missing=1
    fi
    if ! command -v cargo >/dev/null 2>&1; then
        log_error "Rust / Cargo is not installed or not in PATH."
        missing=1
    fi

    if [ "$missing" -eq 1 ]; then
        exit 1
    fi

    if [ ! -d "node_modules" ]; then
        log_warn "node_modules not found. Running npm install..."
        npm install
    fi
}

run_verify() {
    check_dependencies
    local ver
    ver="$(get_version)"
    echo -e "${CYAN}${BOLD}=== Verifying Alitken v${ver} (Linux) ===${NC}"
    
    log_info "Running TypeScript typecheck..."
    npx tsc --noEmit
    log_success "TypeScript check passed cleanly."

    log_info "Running Cargo compile check..."
    cargo check --manifest-path src-tauri/Cargo.toml
    log_success "Cargo check passed cleanly with 0 errors."
}

run_dev() {
    check_dependencies
    local ver
    ver="$(get_version)"
    echo -e "${CYAN}${BOLD}=== Starting Alitken v${ver} Dev Server ===${NC}"
    npx tauri dev
}

run_build() {
    check_dependencies
    local ver
    ver="$(get_version)"
    echo -e "${CYAN}${BOLD}=== Building Alitken v${ver} Release Binary ===${NC}"
    npm run build
    cargo build --release --manifest-path src-tauri/Cargo.toml
    log_success "Binary built successfully at: src-tauri/target/release/alitken-gui"
}

run_rpm() {
    check_dependencies
    local ver
    ver="$(get_version)"
    echo -e "${CYAN}${BOLD}=== Building Fedora RPM Package for Alitken v${ver} ===${NC}"
    
    if ! command -v rpmbuild >/dev/null 2>&1; then
        log_warn "'rpmbuild' is not installed. Installing via 'sudo dnf install -y rpm-build' is recommended."
    fi

    npx tauri build --bundles rpm --ignore-version-mismatches
    log_success "RPM package generated in: src-tauri/target/release/bundle/rpm/"
}

run_appimage() {
    check_dependencies
    local ver
    ver="$(get_version)"
    echo -e "${CYAN}${BOLD}=== Building Universal AppImage for Alitken v${ver} ===${NC}"
    NO_STRIP=1 APPIMAGE_EXTRACT_AND_RUN=1 npx tauri build --bundles appimage --ignore-version-mismatches
    log_success "AppImage generated in: src-tauri/target/release/bundle/appimage/"
}

run_all() {
    run_verify
    run_rpm
    run_appimage
    log_success "All Linux packages (RPM & AppImage) built successfully!"
}

show_menu() {
    local ver
    ver="$(get_version)"
    clear 2>/dev/null || true
    echo -e "${CYAN}${BOLD}================================================================${NC}"
    echo -e "${CYAN}${BOLD}   ⚡ ALITKEN v${ver} - Linux Developer & Packaging Tool ⚡${NC}"
    echo -e "${CYAN}${BOLD}================================================================${NC}"
    echo -e " [1] Start Dev Mode              (tauri dev)"
    echo -e " [2] Run Codebase Verification   (tsc + cargo check)"
    echo -e " [3] Build Release Executable    (standalone binary)"
    echo -e " [4] Package Fedora RPM (.rpm)   (native distribution package)"
    echo -e " [5] Package Universal AppImage  (portable single-file)"
    echo -e " [6] Build All (RPM + AppImage)  (full Linux release)"
    echo -e " [Q] Quit"
    echo -e "${CYAN}----------------------------------------------------------------${NC}"
    read -rp "Select an option [1-6, Q]: " choice
    case "$choice" in
        1) run_dev ;;
        2) run_verify ;;
        3) run_build ;;
        4) run_rpm ;;
        5) run_appimage ;;
        6) run_all ;;
        q|Q) exit 0 ;;
        *) log_warn "Invalid selection."; exit 1 ;;
    esac
}

# Command dispatch
case "$1" in
    dev) run_dev ;;
    verify) run_verify ;;
    build) run_build ;;
    rpm) run_rpm ;;
    appimage) run_appimage ;;
    all) run_all ;;
    "") show_menu ;;
    *) echo "Usage: $0 {dev|verify|build|rpm|appimage|all}"; exit 1 ;;
esac
