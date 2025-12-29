# Algorithm Business Rules (Non-Lookahead Guarantees)

**STATUS**: 📋 Placeholder - Awaiting Implementation

---

## Planned Content

This diagram will show **core business logic guarantees** that make the algorithm trustworthy and the state machine for range bar construction.

### State Machine

1. **New Tick Arrives**
2. **Breach Check Decision**
    - No breach → Update current bar
    - Breach detected → Close current bar

3. **If No Breach**
    - Update OHLCV of current bar
    - Wait for next tick
    - Loop back to step 1

4. **If Breach**
    - Include breach tick in closing bar
    - Output completed bar
    - Start new bar with next tick
    - Wait for next tick
    - Loop back to step 1

### Business Guarantees (Subgraph)

1. **No Future Information Used**
    - Current tick cannot see next tick
    - Decisions made with past + current only

2. **Fixed Thresholds from Open**
    - Threshold computed once when bar opens
    - Never recalculated during bar lifetime

3. **Breach Tick Always Included**
    - Tick that causes breach belongs to closing bar
    - Not to next bar (prevents information loss)

4. **Deterministic Output**
    - Same input → same output, always
    - Platform-independent results

### Planned Diagram Type

Mermaid graph TD showing:

- Circular state machine (tick processing loop)
- Decision node (breach check)
- Two paths (update vs close)
- Guarantee subgraph (4 trust factors)
- Clear distinction between states

---

## Implementation Notes

**Priority**: High (fundamental to understanding system reliability)

**Dependencies**: Deep understanding of algorithm specification

**Estimated Complexity**: Medium (state machine + guarantee visualization)

---

## Temporary Alternative

See [Algorithm Specification](../specifications/algorithm-spec.md) for complete mathematical formulation and guarantees.

---

**Back to Index**: [diagrams/INDEX.md](INDEX.md)
