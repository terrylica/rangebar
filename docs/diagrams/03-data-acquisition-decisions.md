# Data Acquisition Decision Tree

**STATUS**: 📋 Placeholder - Awaiting Implementation

---

## Planned Content

This diagram will show **business logic for choosing data sources and markets**, organized as a decision tree:

### Decision Points

1. **Asset Class?**
    - Cryptocurrency → Binance (Spot/UM/CM)
    - Forex → Exness (Raw_Spread variant)

2. **Market Type?** (if Crypto)
    - Spot trading → Binance Spot
    - Futures USDT → Binance UM Futures
    - Futures Coin → Binance CM Futures

3. **Validation**
    - All paths converge → Data Validation
    - Then → Range Bar Processing

### Planned Diagram Type

Mermaid flowchart showing:

- Binary decision nodes (diamond shapes)
- Market endpoint nodes (rectangles)
- Convergence to unified processing
- Color-coded by asset class

---

## Implementation Notes

**Priority**: High (critical for users choosing data sources)

**Dependencies**: None - straightforward business logic

**Estimated Complexity**: Low (simple decision tree)

---

## Temporary Alternative

For now, see [CLAUDE.md](../../CLAUDE.md) section "Data Source Requirements" which explains the same logic in text.

---

**Back to Index**: [diagrams/INDEX.md](INDEX.md)
