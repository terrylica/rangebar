# Compilation Optimization

Techniques for accelerating Rust compilation in the rangebar workspace.

## Current Configuration

### Active Optimizations

| Technique | Configuration | Location |
|-----------|---------------|----------|
| **Parallel builds** | `jobs = 14` | `~/.cargo/config.toml` |
| **Sparse registry** | `protocol = "sparse"` | `~/.cargo/config.toml` |
| **cargo nextest** | System-wide install | Faster test execution |
| **Modular workspace** | 8 crates | Enables parallel crate compilation |
| **sccache** | `RUSTC_WRAPPER=sccache` | Compilation artifact caching |

### sccache Setup

**Installation**:

```bash
brew install sccache
```

**Configuration** (already in `~/.cargo/config.toml`):

```toml
[build]
rustc-wrapper = "sccache"
```

**Verify**:

```bash
sccache --show-stats
```

**Benefits**:

- Caches compiled artifacts locally (`~/.cache/sccache/`)
- Significant speedup on clean builds when cache is warm
- Optional: Cloud storage backend for team sharing (S3, GCS, Azure Blob)

---

## Profile Configuration

### Release Profile (Maximum Performance)

From `Cargo.toml`:

```toml
[profile.release]
lto = true           # Full link-time optimization
codegen-units = 1    # Better optimization (slower compile)
overflow-checks = false
panic = "abort"      # Smaller binaries
```

### Dev Profile (Faster Iteration)

From `~/.cargo/config.toml`:

```toml
[profile.dev]
debug = true
opt-level = 1        # Some optimization for dev builds
```

---

## Not Configured (With Reasons)

### Fast Linkers (mold/zld)

**Status**: Not applicable for macOS

- **zld**: Deprecated and abandoned by Apple
- **mold**: Linux-only (macOS port `sold` is less mature)
- **ld-prime**: Apple's new linker (Xcode 15+) is already optimized for Apple Silicon

### Cranelift Backend

**Status**: Requires nightly Rust

- The workspace targets stable Rust (`rust-version = "1.90"`)
- Cranelift accelerates debug builds but produces slower binaries
- Consider if debug compilation becomes a bottleneck

---

## Benchmarking Compilation

### Measure Full Build

```bash
# Clean build timing
cargo clean && time cargo build --release

# With sccache stats
sccache --zero-stats
cargo clean && cargo build --release
sccache --show-stats
```

### Measure Incremental Build

```bash
# Touch a core file and rebuild
touch crates/rangebar-core/src/lib.rs
time cargo build --release
```

---

## Additional Techniques

### Remove Unused Dependencies

```bash
# Audit for unused deps
cargo install cargo-machete
cargo machete

# Or use cargo-udeps (requires nightly)
cargo +nightly udeps
```

### Minimize Feature Flags

Review `Cargo.toml` for overly broad feature enablement. Example:

```toml
# ❌ Pulls in everything
polars = { version = "0.51", features = ["all"] }

# ✅ Only what's needed
polars = { version = "0.51", features = ["lazy", "temporal", "parquet"] }
```

### Hardware Optimization

- **SSD**: Ensure `target/` is on fast storage
- **RAM**: 16GB+ recommended for parallel compilation
- **Antivirus**: Exclude `target/` directory from scanning

---

## References

- [The Rust Performance Book - Compile Times](https://nnethercote.github.io/perf-book/compile-times.html)
- [sccache Documentation](https://github.com/mozilla/sccache)
- [cargo-machete](https://github.com/bnjbvr/cargo-machete)
- [Cranelift Backend](https://github.com/rust-lang/rustc_codegen_cranelift)
