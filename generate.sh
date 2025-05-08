#!/usr/bin/env bash

# generate.sh — regenerate Rust models from your OpenAPI spec
# -------------------------------------------------------------
# Usage:
#   ./scripts/generate.sh [options]
#
# Options:
#   -i <spec>           Path to your OpenAPI YAML (default: docs/dist/openapi.yaml)
#   -o <out_dir>        Output directory for generated crate (default: resources)
#   -p <pkg_name>       Cargo packageName for generated crate (default: resources)
#   -g <generator>      Generator name (rust | rust-server | rust-axum; default: rust)
#   -m <global_props>   comma-separated list of Global Properties (default: models,modelDocs=false)
#   -h                  Show this help

set -euo pipefail

# default values
SPEC="docs/dist/openapi.yaml"
OUT_DIR="resources"
PKG_NAME="resources"
GEN="rust"
GLOBAL_PROPS="modelDocs=false,models,supportingFiles"

print_help() {
  grep '^#' "$0" | sed -e 's/^#//'
  exit 0
}

# parse flags
while getopts "i:o:p:g:m:h" opt; do
  case $opt in
    i) SPEC="$OPTARG" ;;
    o) OUT_DIR="$OPTARG" ;;
    p) PKG_NAME="$OPTARG" ;;
    g) GEN="$OPTARG" ;;
    m) GLOBAL_PROPS="$OPTARG" ;;
    h) print_help ;;
    *) print_help ;;
  esac
done

log() {
  echo "[$(date +'%Y-%m-%dT%H:%M:%S%z')] $*"
}

# begin
log "🚀 Starting OpenAPI→Rust generation"
log "Spec       : $SPEC"
log "Output Dir : $OUT_DIR"
log "Generator  : $GEN"
log "Pkg Name   : $PKG_NAME"
log "GlobalProps: $GLOBAL_PROPS"

# rebuild docs
if [ -d docs ]; then
  log "🔨 Building docs (yarn build)…"
  ( cd docs && yarn build ) || { log "❌ yarn build failed"; exit 1; }
else
  log "⚠️  docs/ directory not found; skipping yarn build"
fi

# remove old models
if [ -d "$OUT_DIR" ]; then
  log "🗑️  Removing old generated dir '$OUT_DIR'"
  rm -rf "$OUT_DIR"
fi

# run generator
log "🤖 Running openapi-generator-cli…"
openapi-generator-cli generate \
  -i "$SPEC" \
  -g "$GEN" \
  --global-property="$GLOBAL_PROPS" \
  --additional-properties=packageName="$PKG_NAME" \
  -o "$OUT_DIR" \
  && log "✅ Generation succeeded" \
  || { log "❌ Generation failed"; exit 1; }


log "🗑️  Pruning generated crate…"

find "$OUT_DIR" -maxdepth 1 -type f -not -name 'README.md' -not -name 'Cargo.toml' -exec rm -f {} \;

rm -rf "$OUT_DIR/src/apis"

if [ -f "$OUT_DIR/src/lib.rs" ]; then
  log "✂️  Removing 'pub mod apis;' from lib.rs"
  sed -i '' '/^pub mod apis;/d' "$OUT_DIR/src/lib.rs"
fi

rm -rf "$OUT_DIR/.openapi-generator"

log "✅ Pruned.  You now have:"
tree -L 2 "$OUT_DIR"

log "🎉 Done! Models are in '$OUT_DIR/'"
