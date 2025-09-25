#!/usr/bin/env bash
set -euo pipefail

echo "🔍 Checking for dependency updates..."

# Check current versions
echo "📊 Current major dependencies:"
cargo tree --depth 1 | grep -E "(polars|serde|tokio|rayon|clap)" || true

# Update all dependencies to latest compatible versions
echo "🔄 Updating dependencies..."
cargo update

# Run tests to ensure compatibility
echo "🧪 Running compatibility tests..."
cargo test --quiet

# Check for outdated dependencies
if command -v cargo-outdated >/dev/null 2>&1; then
    echo "📋 Checking for major version updates available:"
    cargo outdated --root-deps-only
else
    echo "💡 Install cargo-outdated for detailed update information:"
    echo "   cargo install cargo-outdated"
fi

# Check Polars features specifically
echo "🐻 Testing Polars features..."
cargo check --features polars-analytics --features polars-io --quiet

echo "✅ Dependencies updated and tested successfully!"