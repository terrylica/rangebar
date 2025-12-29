---
name: release-plz-workflow
description: Automates semantic versioning and release publishing for rangebar Rust workspace using release-plz. Use when preparing releases, bumping versions, generating changelogs, publishing crates to crates.io, or creating GitHub releases.
allowed-tools: Bash(cargo:*), Bash(git:*), Bash(release-plz:*), Bash(gh:*), Bash(doppler:*), Read, Grep, Glob
---

# Release-plz Workflow for Rangebar

## Overview

This skill guides the release-plz workflow for the rangebar 8-crate Rust workspace. It handles semantic versioning with SSoT in `Cargo.toml`, automated changelog generation via git-cliff, and publishing to crates.io in dependency order.

**SSoT**: Version lives only in `Cargo.toml` (workspace-level with inheritance).

## Quick Reference

```bash
# Dry run - preview what will happen
release-plz release --dry-run --git-token "$(gh auth token)"

# Full release
release-plz release --git-token "$(gh auth token)"
```

## Workflow Phases

### Phase 1: Preflight Validation

```bash
# 1. Verify clean working directory
git status --porcelain  # Should be empty

# 2. Verify on main branch
git branch --show-current  # Should be 'main'

# 3. Verify credentials
gh auth status  # Should show terrylica account
doppler secrets get CRATES_IO_CLAUDE_CODE --project claude-config --config dev --plain | head -c 10  # Should show token prefix
```

### Phase 2: Release Execution

```bash
# Export crates.io token
export CARGO_REGISTRY_TOKEN=$(doppler secrets get CRATES_IO_CLAUDE_CODE --project claude-config --config dev --plain)

# Run release-plz with GitHub token
release-plz release --git-token "$(gh auth token)"
```

### Phase 3: Verification

```bash
# Verify tag was created
git tag -l --sort=-version:refname | head -3

# Verify GitHub release
gh release view $(git describe --tags --abbrev=0)

# Verify crates.io
cargo search rangebar
```

## Crate Publication Order

release-plz automatically publishes in dependency order:

1. `rangebar-core` - Core algorithm, fixed-point arithmetic
2. `rangebar-providers` - Data providers (Binance, Exness)
3. `rangebar-config` - Configuration management
4. `rangebar-io` - I/O operations and Polars integration
5. `rangebar-streaming` - Real-time streaming processor
6. `rangebar-batch` - Batch analytics engine
7. `rangebar-cli` - Command-line tools
8. `rangebar` - Meta-crate for backward compatibility

## Troubleshooting

See [Troubleshooting Guide](./references/troubleshooting.md) for common issues.

### Quick Fixes

| Error                              | Solution                                         |
| ---------------------------------- | ------------------------------------------------ |
| "can't determine registry indexes" | `rm -rf ~/.cargo/registry/index/github.com-*`    |
| "git release not configured"       | Add `--git-token "$(gh auth token)"`             |
| CHANGELOG.md warnings per-crate    | Cosmetic; workspace-level changelog works        |
| "already published"                | Crates already on crates.io; create tag manually |

## Configuration Files

| File               | Purpose                                  |
| ------------------ | ---------------------------------------- |
| `release-plz.toml` | release-plz configuration                |
| `cliff.toml`       | git-cliff changelog template             |
| `Cargo.toml`       | SSoT for version (workspace inheritance) |

## Version Determination

release-plz analyzes commits since last tag:

- `feat:` or `feat!:` → MINOR bump
- `fix:` → PATCH bump
- `BREAKING CHANGE:` in body → MAJOR bump
- `chore:`, `docs:`, `refactor:` → No bump (configurable)

## Links

- [release-plz docs](https://release-plz.ieni.dev/)
- [Publishing Guide](/docs/guides/publishing.md)
- [Algorithm Spec](/docs/specifications/algorithm-spec.md)
