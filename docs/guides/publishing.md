# Publishing Guide

Complete guide for publishing rangebar crates to crates.io using release-plz with Doppler credential management.

## Quick Reference

**Tool**: [release-plz](https://release-plz.dev/) - Rust-native release automation

**Doppler Configuration**:

- Project: `claude-config`
- Config: `dev`
- Secret: `CRATES_IO_CLAUDE_CODE`

**Publish Command**:

```bash
export CARGO_REGISTRY_TOKEN=$(doppler secrets get CRATES_IO_CLAUDE_CODE \
  --project claude-config --config dev --plain)
release-plz release
```

**Dry Run** (preview without changes):

```bash
release-plz release --dry-run
```

---

## Authentication Setup

### Doppler Secret Access

The crates.io API token is stored in Doppler secrets management:

```bash
# Retrieve token (requires Doppler CLI authenticated)
doppler secrets get CRATES_IO_CLAUDE_CODE \
  --project claude-config \
  --config dev \
  --plain
```

**Important**: This assumes Doppler CLI is already authenticated. Check authentication:

```bash
cat ~/.doppler/.doppler.yaml
```

### Alternative: Direct Cargo Login

If Doppler is not available:

```bash
cargo login
# Paste your token from https://crates.io/settings/tokens
```

---

## Publication Order

Crates must be published in dependency order:

```bash
# Layer 1: No dependencies
cargo publish -p rangebar-core
cargo publish -p rangebar-config

# Layer 2: Depends on core
cargo publish -p rangebar-providers
cargo publish -p rangebar-io

# Layer 3: Depends on core + layer 2
cargo publish -p rangebar-streaming  # depends: core, providers
cargo publish -p rangebar-batch      # depends: core, io

# Layer 4: Depends on all previous
cargo publish -p rangebar-cli        # depends: all 6 above

# Layer 5: Meta-crate
cargo publish -p rangebar            # depends: all 7 above
```

**Wait Time**: Allow 5-10 seconds between publications for crates.io indexing.

---

## Complete Publishing Workflow

### Pre-Publication Checklist

1. **Clean git state**:

   ```bash
   git status
   # Ensure no uncommitted changes
   ```

2. **API compatibility check**:

   ```bash
   # release-plz runs cargo-semver-checks automatically
   # Manual check if needed:
   cargo semver-checks
   ```

3. **Dry run** (preview release):

   ```bash
   release-plz release --dry-run
   ```

### Automated Full Publication

release-plz handles dependency-ordered publishing automatically:

```bash
# Set credentials
export CARGO_REGISTRY_TOKEN=$(doppler secrets get CRATES_IO_CLAUDE_CODE \
  --project claude-config --config dev --plain)

# Execute release (version bump, changelog, tag, publish)
release-plz release
```

**What release-plz does automatically**:

1. Analyzes commits since last tag (conventional commits)
2. Runs cargo-semver-checks for API breaking change detection
3. Determines version bump (MAJOR/MINOR/PATCH)
4. Updates CHANGELOG.md via git-cliff
5. Creates git tag (v<version>)
6. Publishes all 8 crates in dependency order
7. Creates GitHub release

### Manual Publication (if needed)

If you need to publish individual crates manually:

```bash
export CARGO_REGISTRY_TOKEN=$(doppler secrets get CRATES_IO_CLAUDE_CODE \
  --project claude-config --config dev --plain)

# Publish in dependency order with wait time
cargo publish -p rangebar-core && sleep 10
cargo publish -p rangebar-config && sleep 10
cargo publish -p rangebar-providers && sleep 10
cargo publish -p rangebar-io && sleep 10
cargo publish -p rangebar-streaming && sleep 10
cargo publish -p rangebar-batch && sleep 10
cargo publish -p rangebar-cli && sleep 10
cargo publish -p rangebar && echo "✅ All crates published!"
```

---

## Rate Limits

**crates.io Limits**:

- New publishers: **~6 crates per 12 hours**
- Established publishers: Higher limits

**If Rate Limited**:

```
Error: You have published too many new crates in a short period of time.
Please try again after [timestamp] or email help@crates.io
```

**Solutions**:

1. Wait until the specified timestamp
2. Email <help@crates.io> to request limit increase (include project details)

---

## Troubleshooting

### Error: "no token found"

```bash
# Check if token is in environment
echo $CARGO_REGISTRY_TOKEN

# Re-authenticate
export CARGO_REGISTRY_TOKEN=$(doppler secrets get CRATES_IO_CLAUDE_CODE \
  --project claude-config --config dev --plain)
```

### Error: "all dependencies must have a version requirement"

```toml
# ❌ WRONG
rangebar-core = { path = "../rangebar-core" }

# ✅ CORRECT
rangebar-core = { path = "../rangebar-core", version = "6.0" }
```

### Error: "files in working directory contain changes"

```bash
# Check uncommitted changes
git status

# Commit or use --allow-dirty (not recommended)
git add -A
git commit -m "chore: prepare for publication"
```

### Error: "crate not found in registry"

Wait 30-60 seconds after publishing a dependency before publishing dependent crates:

```bash
cargo publish -p rangebar-core
sleep 30  # Wait for crates.io indexing
cargo publish -p rangebar-providers
```

---

## Verification

### Check Publication Status

```bash
# Via API
curl -s https://crates.io/api/v1/crates/rangebar-core | grep newest_version

# Via browser
open https://crates.io/crates/rangebar-core
```

### Test Installation

```bash
# Create test project
cargo new test-rangebar && cd test-rangebar

# Add dependency
cargo add rangebar-core

# Verify compilation
cargo build
```

---

## Security Considerations

**Doppler Token Storage**:

- ✅ Stored in Doppler secrets (encrypted at rest)
- ✅ Retrieved on-demand (not persisted in environment)
- ✅ Scoped to `claude-config` project

**Token Scope**:

- The `CRATES_IO_CLAUDE_CODE` token has publish permissions
- Rotate token if compromised: <https://crates.io/settings/tokens>

**Git Hygiene**:

- Never commit tokens to git
- Verify `.gitignore` excludes credential files
- Use `git log -S "crates-io"` to check history

---

## CI/CD Integration

**GitHub Actions** (optional backup - see `.github/workflows/release-plz.yml`):

```yaml
name: Release-plz

on:
  push:
    branches: [main]
  workflow_dispatch:

permissions:
  contents: write
  pull-requests: write

jobs:
  release-plz:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - uses: dtolnay/rust-toolchain@stable

      - uses: taiki-e/install-action@release-plz

      - name: Run release-plz
        uses: release-plz/action@v0.5
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          cargo_token: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

**Note**: Primary workflow is local-first with Doppler credentials. GitHub Actions serves as optional backup automation.

---

## References

- [release-plz Documentation](https://release-plz.dev/docs)
- [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks)
- [crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Doppler CLI Documentation](https://docs.doppler.com/docs/cli)
- [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

---

## Changelog

- **2025-12-28**: Migrated to release-plz for SSoT versioning (eliminated dual .cz.toml/Cargo.toml sync)
- **2025-11-12**: Initial publication of 6/8 crates (rate limited on cli + meta-crate)
- **2025-11-12**: Documented Doppler credential access pattern
