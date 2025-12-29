# Threshold Selection Guide

**STATUS**: 📋 Placeholder - Awaiting Implementation

---

## Planned Content

This diagram will show **business logic for choosing optimal threshold** based on trading strategy and expected bar frequency.

### Trading Styles & Thresholds

1. **High-Frequency Trading (HFT)**
   - Threshold: 0.1-0.5 bps
   - Expected: 1000-5000 bars/day
   - Use case: Sub-minute trades

2. **Intraday Trading**
   - Threshold: 0.5-5 bps
   - Expected: 200-1000 bars/day
   - Use case: Minutes to hours

3. **Swing Trading**
   - Threshold: 5-25 bps
   - Expected: 50-200 bars/day
   - Use case: Days to weeks

4. **Position Trading**
   - Threshold: 25-100 bps
   - Expected: 10-50 bars/day
   - Use case: Weeks to months

### Planned Diagram Type

Mermaid flowchart showing:

- Single decision: "Trading Style?"
- 4 branches (HFT, Intraday, Swing, Position)
- Threshold ranges per branch
- Bar frequency outcomes
- Converge to "Execute Strategy"

---

## Implementation Notes

**Priority**: High (practical decision-making for users)

**Dependencies**: None - straightforward mapping

**Estimated Complexity**: Low (linear flow, no complex logic)

---

## Temporary Alternative

See [Common Workflows Guide](/docs/guides/common-workflows.md) Workflow 5: "Find Optimal Threshold for Symbol" for code examples.

---

**Back to Index**: [diagrams/INDEX.md](/docs/diagrams/INDEX.md)
