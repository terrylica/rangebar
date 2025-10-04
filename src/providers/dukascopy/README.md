# Dukascopy Provider (DEPRECATED)

**Status**: ⚠️ DEPRECATED as of 2025-10-03
**Replacement**: `exness` provider

---

## Deprecation Reason

Dukascopy data source exhibits severe rate limiting issues preventing reliable data fetching:

- **77.5% success rate**: 27/120 hourly requests failed with HTTP 503
- **Complex timeout requirements**: 120s request timeout, 30s connect timeout
- **IP-based rate limiting**: Long windows, unpredictable blocking
- **Slow downloads**: 250 seconds for 5 days of data
- **Implementation complexity**: LZMA decompression, binary parsing, instrument config (1,607 instruments)

**Root cause**: Dukascopy server enforces undocumented rate limiting despite conservative request spacing (2s between requests).

**Validation**: Reference implementation (`dukascopy-node`) succeeded with identical parameters, proving issue is Dukascopy server restrictions, not data quality.

**Reference**: `docs/planning/dukascopy-timeout-retry-strategy.md`

---

## Migration Path

**Use Exness Raw_Spread variant instead**:

✅ **Advantages**:
- 100% reliability (zero rate limiting observed)
- 3 seconds/month download (80× faster than Dukascopy)
- Simpler implementation (ZIP/CSV vs LZMA/binary)
- Out-of-box dependencies (no custom parsers)
- Same Jan 15-19, 2024 test: 300,425 ticks (vs 418,138 Dukascopy)

⚠️ **Trade-offs**:
- 28% fewer ticks (60K/day vs 84K/day Dukascopy)
- No tick volumes (Bid/Ask prices only)

**Verdict**: Trade-offs acceptable for range bar generation (480 bars/day target easily met).

**See**:
- `src/providers/exness/` (implementation)
- `docs/planning/research/exness-eurusd-variant-analysis.md` (data quality validation)
- `docs/planning/exness-migration-plan.md` (migration guide)

---

## Code Preservation Rationale

This module is **preserved** (not deleted) for:

1. **Historical reference**: Binary format parsing patterns
2. **Volume research**: Future volume-weighted range bar features
3. **Multi-source validation**: Cross-provider data quality checks
4. **Rollback capability**: Emergency fallback if Exness fails

**Do NOT use for new implementations**.

---

## Technical Details (For Reference)

**Data Format**: LZMA-compressed binary (.bi5)
**Structure**: 20 bytes/tick (bid, ask, bid_volume, ask_volume, timestamp_ms)
**Granularity**: Hourly files
**API**: `https://datafeed.dukascopy.com/datafeed/{instrument}/{year}/{month}/{day}/{hour}h_ticks.bi5`

**Dependencies**:
- `lzma-rs`: LZMA decompression
- `byteorder`: Big-endian binary parsing
- `toml`: Instrument config (1,607 instruments)
- `once_cell`: Lazy static config

**Complexity**: 47 KB across 4 files (client.rs, types.rs, builder.rs, conversion.rs)

---

## Historical Performance Data

**Phase 1 (100ms delay)**:
- Requests: 120 (Jan 15-19, 2024, hourly)
- Success: 93/120 (77.5%)
- Failures: 27 (HTTP 503)
- Data fetched: 357,859 ticks (~2.4 days)

**Phase 2 (2s delay)**:
- Requests: 120
- Success: 0/120 (0%)
- Failure: Immediate HTTP 503 on first request
- Hypothesis: IP rate limit carryover

**Benchmark (dukascopy-node)**:
- Requests: 120
- Success: 120/120 (100%)
- Data fetched: 418,138 ticks (full 5 days)
- Time: 249.6 seconds

**Conclusion**: Dukascopy server functional, but our implementation blocked by rate limits despite conservative delays.

---

## Migration Timeline

- **v2.3.0** (2025-10-03): Dukascopy marked deprecated, Exness added
- **v3.0.0** (planned): Remove Dukascopy provider entirely
- **Support window**: 1 release cycle

**Action Required**: Migrate all forex tick data sources to Exness Raw_Spread.
