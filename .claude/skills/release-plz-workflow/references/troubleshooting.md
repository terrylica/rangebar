# Troubleshooting Guide

Lessons learned from production release-plz usage with the rangebar workspace.

## Common Issues

### 1. "can't determine registry indexes"

**Symptom**:

```
ERROR can't determine registry indexes

Caused by:
    Could not obtain the most recent head commit in repo at
    /Users/terryli/.cargo/registry/index/github.com-1ecc6299db9ec823
```

**Root Cause**: Old git-based cargo registry index is corrupted or incompatible with modern cargo.

**Solution**:

```bash
rm -rf ~/.cargo/registry/index/github.com-1ecc6299db9ec823
```

**Why this works**: Cargo now uses sparse registry protocol by default. The old git-based index is no longer needed and can be safely removed.

---

### 2. "git release not configured"

**Symptom**: release-plz runs but doesn't create GitHub releases or git tags.

**Root Cause**: Missing GitHub token for API operations.

**Solution**:

```bash
/usr/bin/env bash << 'FIX2_EOF'
release-plz release --git-token "$(gh auth token)"
FIX2_EOF
```

**Alternative** (environment variable):

```bash
/usr/bin/env bash << 'FIX2_ALT_EOF'
export GITHUB_TOKEN=$(gh auth token)
release-plz release
FIX2_ALT_EOF
```

---

### 3. CHANGELOG.md Warnings Per-Crate

**Symptom**:

```
WARN rangebar-core: failed to parse changelog at path
"/Users/terryli/eon/rangebar/crates/rangebar-core/CHANGELOG.md":
can't read changelog file
```

**Root Cause**: release-plz looks for per-crate CHANGELOG.md files.

**Solution**: These warnings are cosmetic. The workspace-level `CHANGELOG.md` at the repo root works fine. release-plz will still:

- Create git tags
- Create GitHub releases
- Publish to crates.io

**Optional fix**: Create empty CHANGELOG.md in each crate directory if warnings bother you.

---

### 4. "already published"

**Symptom**:

```
INFO rangebar-core 5.0.0: already published
INFO rangebar-providers 5.0.0: already published
...
```

**Root Cause**: Crates are already on crates.io at this version, but no git tag exists.

**Solution**: Manually create the tag and GitHub release:

```bash
# Create annotated tag
git tag -a v5.0.0 -m "Release v5.0.0"

# Push tag
git push origin v5.0.0

# Create GitHub release
gh release create v5.0.0 --title "RangeBar v5.0.0" --generate-notes
```

---

### 5. Doppler Token Not Found

**Symptom**: `cargo publish` fails with authentication error.

**Root Cause**: CARGO_REGISTRY_TOKEN not set or expired.

**Solution**:

```bash
/usr/bin/env bash << 'FIX5_EOF'
# Verify Doppler access
doppler secrets get CRATES_IO_CLAUDE_CODE --project claude-config --config dev --plain

# If it works, export it
export CARGO_REGISTRY_TOKEN=$(doppler secrets get CRATES_IO_CLAUDE_CODE --project claude-config --config dev --plain)
FIX5_EOF
```

**If Doppler fails**: Re-authenticate with `doppler login`.

---

### 6. Wrong GitHub Account

**Symptom**: Releases created under wrong account.

**Root Cause**: Multiple GitHub accounts configured.

**Solution**:

```bash
# Check current account
gh auth status

# If wrong, re-authenticate
gh auth login
```

---

### 7. Version Mismatch Between Crates

**Symptom**: Some crates have different versions.

**Root Cause**: Not using workspace version inheritance.

**Solution**: All crates should use:

```toml
# In each crate's Cargo.toml
[package]
version.workspace = true
```

And the workspace Cargo.toml should define:

```toml
[workspace.package]
version = "5.0.0"
```

---

### 8. Orphaned Git Submodule

**Symptom**:

```
error: failed to retrieve git status from repo /path/.git/modules/repos/binance-public-data

Caused by:
  Worktree root at '/path/repos/binance-public-data' is not a directory
```

**Root Cause**: `.gitmodules` references a submodule whose directory was deleted but git metadata remains.

**Solution**:

```bash
# Remove orphaned submodule completely
git submodule deinit -f repos/binance-public-data
rm -rf .git/modules/repos/binance-public-data
rm .gitmodules  # If no other submodules remain
git add -A && git commit --no-verify -m "chore: remove orphaned submodule"
git push origin main
```

---

### 9. Partial Release (Tag Already Exists)

**Symptom**:

```
ERROR failed to create ref refs/tags/v5.0.1
Response body: {"message":"Reference already exists","status":"422"}
```

**Root Cause**: release-plz created the tag but failed afterward; only some crates were published.

**Diagnosis**:

```bash
# Check which crates were published
cargo search rangebar --limit 8  # Compare versions - some may be old
```

**Solution**: Manually publish remaining crates:

```bash
/usr/bin/env bash << 'PARTIAL_EOF'
export CARGO_REGISTRY_TOKEN=$(doppler secrets get CRATES_IO_CLAUDE_CODE --project claude-config --config dev --plain)

# Publish remaining crates in dependency order (adjust list based on diagnosis)
for crate in rangebar-config rangebar-io rangebar-streaming rangebar-batch rangebar-cli rangebar; do
  echo "Publishing $crate..."
  cargo publish -p $crate --allow-dirty
  sleep 10  # Wait for crates.io index
done
PARTIAL_EOF
```

---

### 10. cargo-deny Advisory DB Error (CVSS 4.0)

**Symptom** (in pre-commit hook):

```
ERROR failed to load advisory database: parse error:
unsupported CVSS version: 4.0
```

**Root Cause**: The RustSec advisory database has entries with CVSS 4.0 scores that cargo-deny doesn't support yet.

**Solution**: Use `--no-verify` flag for commits until cargo-deny is updated:

```bash
git commit --no-verify -m "your commit message"
```

**Note**: This is a temporary workaround. The cargo-deny maintainers need to add CVSS 4.0 support.

---

## Pre-Release Checklist

Before running `release-plz release`:

1. [ ] `git status --porcelain` is empty (clean working directory)
2. [ ] On `main` branch
3. [ ] `gh auth status` shows correct account (terrylica)
4. [ ] Doppler token works: `doppler secrets get CRATES_IO_CLAUDE_CODE --project claude-config --config dev --plain`
5. [ ] All tests pass: `cargo test`
6. [ ] No clippy warnings: `cargo clippy`
7. [ ] No orphaned submodules: `git submodule status`
8. [ ] Target version tag doesn't exist: `git tag -l "v<version>"`

---

## Environment Variables

| Variable               | Source          | Purpose                  |
| ---------------------- | --------------- | ------------------------ |
| `CARGO_REGISTRY_TOKEN` | Doppler         | crates.io authentication |
| `GITHUB_TOKEN`         | `gh auth token` | GitHub releases/tags     |
| `GH_TOKEN`             | mise `[env]`    | Multi-account GitHub     |

---

## Useful Commands

```bash
/usr/bin/env bash << 'USEFUL_EOF'
# Preview release without making changes
release-plz release --dry-run --git-token "$(gh auth token)"

# Check what version would be bumped
release-plz get-manifest

# View recent tags
git tag -l --sort=-version:refname | head -5

# View GitHub releases
gh release list --limit 5

# Check crates.io versions
cargo search rangebar
USEFUL_EOF
```
