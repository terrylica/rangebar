# Dukascopy Timeout and Retry Strategy

**Version**: 1.0.0
**Created**: 2025-10-03
**Status**: PHASE-2-TESTING
**Type**: CRITICAL-FIX
**Supersedes**: `docs/planning/dukascopy-rate-limit-mitigation.md` (100ms delay insufficient)
**References**:
- `src/providers/dukascopy/client.rs` (HTTP client configuration)
- `tests/dukascopy_eurusd_adversarial_audit.rs` (audit_7 test)

---

## Problem Statement

**Observed Failures**:
1. Test aborts at request 2/120 with `error sending request for url`
2. Previous 100ms delay mitigation insufficient
3. Dukascopy response time: 15-45 seconds (highly variable)
4. HTTP 503 still occurs even with 100ms delays

**Empirical Data** (curl measurements):
```
Hour 00: 42.5s → HTTP 200 ✅
Hour 01: 0.5s  → HTTP 503 ❌ (100ms delay insufficient)
Hour 02: 16.0s → HTTP 200 ✅
```

**Root Causes**:
1. **Client timeout**: reqwest default 90s, but premature failures suggest issues
2. **Rate limit window**: 100ms delay inadequate for Dukascopy's rate limit policy
3. **No retry logic**: Transient failures abort entire test

---

## Dukascopy Server Characteristics

### Response Time Distribution
- **Minimum**: 0.5s (503 error responses)
- **Typical**: 15-18s (successful data fetch)
- **Maximum Observed**: 42.5s
- **Design Assumption**: 60s max (safety margin)

### Rate Limit Behavior
- **Type**: Request-count-per-time-window (exact window unknown)
- **Observation**: 100ms delay triggers immediate 503 after 42s request
- **Hypothesis**: Minimum 1-2 second spacing required between requests
- **Recovery**: Subsequent request succeeds after 503 (window resets)

### Error Types
**Retryable** (transient):
- HTTP 503 Service Unavailable (rate limit)
- Connection timeout (server slow response)
- `error sending request` (network transient)

**Non-Retryable** (permanent):
- HTTP 404 Not Found (data doesn't exist)
- HTTP 401/403 (authentication/authorization)
- HTTP 400 (malformed request)

---

## Proposed Solution: 3-Layer Strategy

### Layer 1: HTTP Client Timeout Configuration

**Current**: `Client::new()` (90s default timeout)

**Proposed**: Explicit timeout configuration
```rust
reqwest::Client::builder()
    .timeout(Duration::from_secs(120))        // Total request: 120s (2x observed max)
    .connect_timeout(Duration::from_secs(30)) // Connection: 30s
    .build()
```

**Rationale**:
- Observed max: 42.5s
- Safety margin: 2.8x headroom (120s / 42.5s)
- Connection timeout: Separate from data transfer timeout

**File**: `src/providers/dukascopy/client.rs:133`

---

### Layer 2: Base Request Delay

**Current**: 100ms (INSUFFICIENT)

**Proposed**: 2000ms (2 seconds)

**Rationale**:
- Empirical: 100ms triggers 503
- Industry standard (dukascopy-node): 1000ms for fast servers
- Dukascopy is 150-400x slower → 2x industry standard = 2000ms
- Conservative approach to respect rate limits

**File**: `tests/dukascopy_eurusd_adversarial_audit.rs:411`

---

### Layer 3: Exponential Backoff Retry (HTTP 503 Only)

**Trigger**: HTTP 503 Service Unavailable

**Strategy**: Exponential with cap
```
Attempt 1: Original request
Attempt 2: Wait  5s, retry (total:  5s)
Attempt 3: Wait 10s, retry (total: 15s)
Attempt 4: Wait 20s, retry (total: 35s)
Attempt 5: Wait 30s, retry (total: 65s)
Max: 5 attempts, ~65s total backoff
```

**Backoff Formula**: `min(5 * 2^(attempt-1), 30)` seconds

**Implementation**:
```rust
async fn fetch_with_retry(
    client: &Client,
    url: &str,
    max_retries: u32,
) -> Result<Bytes, DukascopyError> {
    let mut attempt = 0;

    loop {
        attempt += 1;

        match client.get(url).send().await {
            Ok(response) if response.status() == 503 => {
                if attempt >= max_retries {
                    return Err(DukascopyError::RateLimitExceeded {
                        attempts: attempt,
                        last_error: "HTTP 503 after max retries".to_string(),
                    });
                }

                let backoff_secs = (5 * 2_u64.pow(attempt - 1)).min(30);
                eprintln!("⚠️  HTTP 503 on attempt {}/{}. Backoff: {}s",
                         attempt, max_retries, backoff_secs);
                tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
                continue;
            }
            Ok(response) => return Ok(response.bytes().await?),
            Err(e) if attempt >= max_retries => {
                return Err(DukascopyError::NetworkError {
                    attempts: attempt,
                    source: e,
                });
            }
            Err(e) if e.is_timeout() || e.is_connect() => {
                let backoff_secs = (5 * 2_u64.pow(attempt - 1)).min(30);
                eprintln!("⚠️  Network error on attempt {}/{}. Backoff: {}s. Error: {}",
                         attempt, max_retries, backoff_secs, e);
                tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
                continue;
            }
            Err(e) => {
                // Non-retryable error (404, 400, etc.)
                return Err(DukascopyError::HttpError { source: e });
            }
        }
    }
}
```

**File**: `src/providers/dukascopy/client.rs` (new function)

---

## SLO Definitions

### Availability
- **Target**: 100% fetch success rate (120/120 requests)
- **Current**: 0.8% (1/120 before abort)
- **Metric**: `successful_fetches / total_requests`
- **Validation**: All requests complete without panic

### Correctness
- **Target**: All fetched data valid, no corruption from retries
- **Metric**: Temporal integrity check passes on all bars
- **Invariant**: Retry logic must not alter response data
- **Validation**: `assert!(bars.iter().all(|b| b.close_time >= b.open_time))`

### Observability
- **Target**: Full visibility into retry behavior and failures
- **Metrics**:
  - Request attempt count per fetch
  - Backoff duration per retry
  - Total retry time per request
  - Final success/failure reason
- **Output Format**:
  ```
  ⚠️  HTTP 503 on attempt 2/5. Backoff: 5s
  ✅ Fetched 3102 ticks for 2024-01-15 00:00 (request 1/120, 1 attempt, 42.5s total)
  ```

### Maintainability
- **Target**: Single source of truth for retry configuration
- **Constants**:
  ```rust
  const DUKASCOPY_REQUEST_TIMEOUT_SECS: u64 = 120;
  const DUKASCOPY_CONNECT_TIMEOUT_SECS: u64 = 30;
  const DUKASCOPY_BASE_DELAY_MS: u64 = 2000;
  const DUKASCOPY_MAX_RETRIES: u32 = 5;
  const DUKASCOPY_BACKOFF_BASE_SECS: u64 = 5;
  const DUKASCOPY_BACKOFF_MAX_SECS: u64 = 30;
  ```
- **Location**: Top of `DukascopyFetcher` implementation

---

## Implementation Plan

### Phase 1: Update HTTP Client Timeout (Critical Path)

**File**: `src/providers/dukascopy/client.rs`

**Current Code** (line 133):
```rust
client: Client::new(),
```

**New Code**:
```rust
// Dukascopy-specific timeout configuration
// Empirical data: 15-45s typical, 120s safety margin (2.8x max observed)
// Reference: docs/planning/dukascopy-timeout-retry-strategy.md
const DUKASCOPY_REQUEST_TIMEOUT_SECS: u64 = 120;
const DUKASCOPY_CONNECT_TIMEOUT_SECS: u64 = 30;

client: Client::builder()
    .timeout(Duration::from_secs(DUKASCOPY_REQUEST_TIMEOUT_SECS))
    .connect_timeout(Duration::from_secs(DUKASCOPY_CONNECT_TIMEOUT_SECS))
    .build()
    .expect("Failed to build Dukascopy HTTP client"),
```

**Error Handling**: `expect()` on client builder (construction should never fail with valid constants)

---

### Phase 2: Increase Base Delay (Test Impact Assessment)

**File**: `tests/dukascopy_eurusd_adversarial_audit.rs:411`

**Current**: `const DUKASCOPY_RATE_LIMIT_DELAY_MS: u64 = 100;`

**New**: `const DUKASCOPY_RATE_LIMIT_DELAY_MS: u64 = 2000;`

**Impact**:
- Test duration increase: 120 requests × (2000ms - 100ms) = +228 seconds = +3.8 minutes
- Previous: ~22 minutes
- New: ~26 minutes
- Acceptable for `#[ignore]` test (manual execution only)

**Validation After Phase 2**: Run test, check if 2s delay prevents 503 errors

---

### Phase 3: Add Exponential Backoff Retry (If Phase 2 Insufficient)

**File**: `src/providers/dukascopy/client.rs`

**Steps**:
1. Add `fetch_with_retry()` function (as specified above)
2. Update `fetch_hour()` to use retry wrapper
3. Add retry configuration constants
4. Add observability logs for retry attempts

**Conditional**: Only implement if Phase 2 still shows 503 errors

---

## Error Propagation Strategy

### Error Types (New Enum Required)

**File**: `src/providers/dukascopy/error.rs` (create if not exists)

```rust
#[derive(Debug, thiserror::Error)]
pub enum DukascopyError {
    #[error("Rate limit exceeded after {attempts} attempts: {last_error}")]
    RateLimitExceeded {
        attempts: u32,
        last_error: String,
    },

    #[error("Network error after {attempts} attempts")]
    NetworkError {
        attempts: u32,
        #[source]
        source: reqwest::Error,
    },

    #[error("HTTP error")]
    HttpError {
        #[source]
        source: reqwest::Error,
    },

    #[error("Decompression failed")]
    DecompressionError {
        #[source]
        source: std::io::Error,
    },

    #[error("Parse error: {context}")]
    ParseError {
        context: String,
    },
}
```

### Propagation Rules

**Retryable Errors** → Retry up to max, then propagate as `RateLimitExceeded` or `NetworkError`

**Non-Retryable Errors** → Immediate propagation as `HttpError`

**Test Layer** → All errors propagate to `panic!()` (fail-fast, no silent handling)

---

## Validation Strategy

### Phase 1 Validation (Timeout Only)
```bash
# Expect: May still fail on 503, but no timeout errors
cargo test audit_7 -- --ignored --nocapture
```

**Success Criteria**:
- ✅ No `error sending request` (timeout) errors
- ✅ May see HTTP 503 (indicates timeout fix worked)
- ❌ Still failing → proceed to Phase 2

### Phase 2 Validation (Timeout + 2s Delay)
```bash
cargo test audit_7 -- --ignored --nocapture
```

**Success Criteria**:
- ✅ 120/120 requests successful
- ✅ 0 HTTP 503 errors
- ✅ ~500K-600K ticks fetched
- ✅ Test duration ~26 minutes
- ❌ Still seeing 503 → proceed to Phase 3

### Phase 3 Validation (Full Retry Logic)
```bash
cargo test audit_7 -- --ignored --nocapture
```

**Success Criteria**:
- ✅ 120/120 requests successful (after retries)
- ✅ Retry logs visible for any 503 occurrences
- ✅ No panics, all errors recovered
- ✅ ~500K-600K ticks fetched

---

## Risk Assessment

### Risk 1: 2s Delay Still Insufficient
**Likelihood**: Medium (unknown exact rate limit)
**Impact**: High (continued 503 errors)
**Mitigation**: Phase 3 retry logic handles 503 gracefully
**Detection**: Test output shows HTTP 503 errors

### Risk 2: Timeout Too Long (120s)
**Likelihood**: Low (conservative vs 42.5s max observed)
**Impact**: Low (only delays failure detection)
**Mitigation**: None needed (acceptable trade-off)

### Risk 3: Retry Logic Introduces Bugs
**Likelihood**: Medium (new async code path)
**Impact**: High (data corruption or hangs)
**Mitigation**:
- Strict error type matching (only retry 503/timeout)
- Max retry cap (5 attempts, ~65s total)
- Comprehensive logging for observability
**Detection**: Temporal integrity validation fails

### Risk 4: Test Duration Excessive
**Likelihood**: Medium (26 min base + retries)
**Impact**: Low (manual `#[ignore]` test only)
**Mitigation**: Accept longer duration for reliability
**Detection**: User feedback on test runtime

---

## Alternative Approaches Rejected

### Alternative 1: Reduce Data Volume
**Approach**: Fetch fewer hours (e.g., 60 instead of 120)
**Rejected**: Defeats purpose of comprehensive audit, doesn't solve root cause

### Alternative 2: Parallel Requests with Semaphore
**Approach**: Concurrent fetches with rate limit semaphore
**Rejected**:
- More complex (semaphore + async coordination)
- May still trigger rate limits (burst detection)
- Harder to debug/observe

### Alternative 3: Pre-Cache Data Locally
**Approach**: Download once, reuse for tests
**Rejected**:
- Defeats purpose of live data validation
- Introduces stale data concerns
- Not applicable to production use

### Alternative 4: Use Official Dukascopy SDK
**Approach**: Switch to JForex SDK or official client
**Rejected**:
- Not Rust-native (JVM dependency)
- Heavier than needed (full trading platform)
- Current solution viable with proper config

---

## Success Criteria

### Must Have (P0)
- ✅ Zero request failures (120/120 success)
- ✅ No timeout errors
- ✅ No unhandled 503 errors
- ✅ All data integrity checks pass
- ✅ Test completes without panic

### Should Have (P1)
- ✅ Test duration < 30 minutes
- ✅ Retry observability (logs for all retry attempts)
- ✅ Clear error messages on final failure
- ✅ Configuration constants documented

### Nice to Have (P2)
- ✅ Retry statistics summary at test end
- ✅ Performance metrics (avg response time, retry rate)
- ✅ Configurable retry policy via constants

---

## Execution Checklist

### Pre-Implementation
- [ ] Review empirical data (curl measurements confirmed)
- [ ] Verify reqwest version supports timeout configuration
- [ ] Check if DukascopyError enum exists (create if needed)

### Phase 1: Timeout Configuration
- [ ] Add timeout constants to `client.rs`
- [ ] Update `Client::new()` to `Client::builder()`
- [ ] Set `.timeout()` and `.connect_timeout()`
- [ ] Compile and verify no errors
- [ ] Run test: `cargo test audit_7 -- --ignored --nocapture`
- [ ] Document results (timeout errors fixed?)

### Phase 2: Increase Base Delay
- [ ] Update `DUKASCOPY_RATE_LIMIT_DELAY_MS` from 100 to 2000
- [ ] Run test: `cargo test audit_7 -- --ignored --nocapture`
- [ ] Document results (503 errors eliminated?)
- [ ] If success → mark COMPLETE, skip Phase 3
- [ ] If failure → proceed to Phase 3

### Phase 3: Exponential Backoff (Conditional)
- [ ] Create/update `DukascopyError` enum
- [ ] Implement `fetch_with_retry()` function
- [ ] Update `fetch_hour()` to use retry wrapper
- [ ] Add retry constants
- [ ] Add retry observability logs
- [ ] Run test: `cargo test audit_7 -- --ignored --nocapture`
- [ ] Verify 120/120 success with retry logs

### Post-Implementation
- [ ] Update `dukascopy-rate-limit-mitigation.md` status to SUPERSEDED
- [ ] Update `NEXT_STEPS.md` with resolution status
- [ ] Mark this plan as COMPLETE
- [ ] Commit: `fix(dukascopy): add timeout config, 2s delay, retry logic for 503`

---

## Observability Metrics

### Pre-Implementation Baseline
- Fetch success: 1/120 (0.8%)
- First failure: Request 2 (`error sending request`)
- No retry attempts (immediate abort)

### Post-Phase 1 Target (Timeout Fix)
- Fetch success: Variable (may still see 503)
- Timeout errors: 0
- Metric: No more `error sending request` failures

### Post-Phase 2 Target (2s Delay)
- Fetch success: 120/120 (100%)
- HTTP 503 errors: 0
- Test duration: ~26 minutes

### Post-Phase 3 Target (Full Retry)
- Fetch success: 120/120 (100%, after retries)
- Retry rate: < 10% (< 12 requests needed retries)
- Avg retries per failed request: < 2
- Total retry time: < 5 minutes

### Monitoring Output Format
```
📊 Fetch statistics:
  ✅ Success: 120/120 (100.0%)
  ❌ Failed: 0
  🔄 Retries: 8 requests needed retry (6.7%)
  ⏱️  Avg retry time: 8.3s per retried request
  📈 Total fetch time: 27m 14s
```

---

## References

**Empirical Data**:
- curl measurements: 42.5s, 0.5s (503), 16.0s response times
- Previous test: 1/120 success before abort
- Rate limit: 100ms delay insufficient

**External**:
- reqwest timeout docs: https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html#method.timeout
- Exponential backoff pattern: https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/
- dukascopy-node: https://github.com/Leo4815162342/dukascopy-node (1000ms delay reference)

**Internal**:
- HTTP client: `src/providers/dukascopy/client.rs`
- Test file: `tests/dukascopy_eurusd_adversarial_audit.rs`
- Previous plan: `docs/planning/dukascopy-rate-limit-mitigation.md` (now superseded)
- Error types: `src/providers/dukascopy/error.rs` (TBD if exists)
