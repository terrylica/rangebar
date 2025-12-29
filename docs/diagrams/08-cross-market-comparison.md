# Cross-Market Comparison Workflow

**STATUS**: 📋 Placeholder - Awaiting Implementation

---

## Planned Content

This diagram will show **business logic for comparing liquidity and volatility across Spot/UM/CM markets**.

### Workflow Stages

1. **Symbol Selection**
   - Choose target symbol (e.g., BTCUSDT)

2. **Parallel Data Fetching**
   - Fetch Spot data
   - Fetch UM Futures data
   - Fetch CM Futures data
   - All 3 in parallel (efficiency)

3. **Parallel Processing**
   - Generate bars: Spot
   - Generate bars: UM Futures
   - Generate bars: CM Futures
   - Same threshold for all (apples-to-apples)

4. **Statistics Calculation**
   - Per market: Volume, Duration, Count
   - Calculate for all 3 markets

5. **Comparison Analysis**
   - Compare metrics across markets
   - Generate insights

6. **Business Insights**
   - Liquidity ranking (which market most liquid)
   - Volatility patterns (which market most volatile)
   - Optimal market selection (recommendation)

### Planned Diagram Type

Mermaid graph TD showing:

- Single start (symbol selection)
- 3 parallel branches (one per market)
- Convergence at comparison node
- 3 insight outcomes
- Clear parallelism visualization

---

## Implementation Notes

**Priority**: Medium (specific to multi-market analysis use case)

**Dependencies**: Understanding of parallel processing benefits

**Estimated Complexity**: Medium (parallel branches require clear layout)

---

## Temporary Alternative

See [Common Workflows Guide](/docs/guides/common-workflows.md) Workflow 7: "Spot vs Futures Comparison" for code example.

---

**Back to Index**: [diagrams/INDEX.md](/docs/diagrams/INDEX.md)
