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

```shell
rm -rf ~/.cargo/registry/index/github.com-1ecc6299db9ec823
```

**Why this works**: Cargo now uses sparse registry protocol by default. The old git-based index is no longer needed and can be safely removed.

---

### 2. "git release not configured"

**Symptom**: release-plz runs but doesn't create GitHub releases or git tags.

**Root Cause**: Missing GitHub token for API operations.

**Solution**:

```shell
release-plz release --git-token "$(gh auth token)"
```

**Alternative** (environment variable):

```shell
export GITHUB_TOKEN=$(gh auth token)
release-plz release
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

```shell
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

```shell
# Verify Doppler access
doppler secrets get CRATES_IO_CLAUDE_CODE --project claude-config --config dev --plain

# If it works, export it
export CARGO_REGISTRY_TOKEN=$(doppler secrets get CRATES_IO_CLAUDE_CODE --project claude-config --config dev --plain)
```

**If Doppler fails**: Re-authenticate with `doppler login`.

---

### 6. Wrong GitHub Account

**Symptom**: Releases created under wrong account.

**Root Cause**: Multiple GitHub accounts configured.

**Solution**:

```shell
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

## Pre-Release Checklist

Before running `release-plz release`:

1. [ ] `git status --porcelain` is empty (clean working directory)
2. [ ] On `main` branch
3. [ ] `gh auth status` shows correct account (terrylica)
4. [ ] Doppler token works: `doppler secrets get CRATES_IO_CLAUDE_CODE --project claude-config --config dev --plain`
5. [ ] All tests pass: `cargo test`
6. [ ] No clippy warnings: `cargo clippy`

---

## Environment Variables

| Variable               | Source          | Purpose                  |
| ---------------------- | --------------- | ------------------------ |
| `CARGO_REGISTRY_TOKEN` | Doppler         | crates.io authentication |
| `GITHUB_TOKEN`         | `gh auth token` | GitHub releases/tags     |
| `GH_TOKEN`             | mise `[env]`    | Multi-account GitHub     |

---

## Useful Commands

```shell
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
```
