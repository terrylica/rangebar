# Planning Documentation Index

**Last Updated**: 2026-01-09
**Total Documents**: 25 (18 markdown, 7 YAML)
**Purpose**: Central navigation hub for all planning, architecture, and research documentation

---

## Quick Navigation

- [Active Plans](#active-plans) - Currently being implemented
- [Completed Plans](#completed-plans) - Finished and delivered
- [Reference Documents](#reference-documents) - Quick reference guides
- [By Category](#by-category) - Organized by type

---

## Active Plans

### Current Implementation

- [`PRODUCTION_HARDENING_ROADMAP.md`](/docs/planning/PRODUCTION_HARDENING_ROADMAP.md) - Production readiness improvements
- [`COMPREHENSIVE-TODO-LIST.md`](/docs/planning/current/COMPREHENSIVE-TODO-LIST.md) - Master tracking for repository audit

### Current Planning

- [`walk-forward-pattern-recognition-system.md`](/docs/planning/current/walk-forward-pattern-recognition-system.md) - ML pattern recognition system
- [`quick-wins-plan.md`](/docs/planning/current/quick-wins-plan.md) - Quick improvement opportunities
- [`repository-audit-plan.md`](/docs/planning/current/repository-audit-plan.md) - Systematic audit plan

### YAML Plans (Current)

- [`code-consolidation-plan.yml`](/docs/planning/current/code-consolidation-plan.yml) - DRY principle implementation
- [`data-structure-validation-plan.yml`](/docs/planning/current/data-structure-validation-plan.yml) - Cross-market validation
- [`historical-replay-visualizer.yml`](/docs/planning/current/historical-replay-visualizer.yml) - Historical data replay system
- [`codebase-sanitization-plan.yml`](/docs/planning/current/codebase-sanitization-plan.yml) - Codebase cleanup

---

## Completed Plans

### Migrations & Upgrades

- [`workspace-migration-v5.0.0.md`](/docs/planning/workspace-migration-v5.0.0.md) - v4→v5 workspace restructure (Complete)
- [`restructure-v2.3.0-migration.md`](/docs/planning/architecture/restructure-v2.3.0-migration.md) - v2.3.0 restructuring (Complete)
- [`api-threshold-granularity-migration.md`](/docs/planning/api-threshold-granularity-migration.md) - Threshold 1bps→0.1bps migration
- [`dukascopy-cleanup-plan-v5.0.0.md`](/docs/planning/dukascopy-cleanup-plan-v5.0.0.md) - Dukascopy provider removal

### Cleanup & Refactoring

- [`test-cleanup-plan-v2-llm-friendly.md`](/docs/planning/test-cleanup-plan-v2-llm-friendly.md) - Test refactoring (LLM-friendly approach)
- [`hybrid-plan-phase1.md`](/docs/planning/current/hybrid-plan-phase1.md) - Phase 1 hybrid plan (Complete)
- [`test-refactoring-plan.yml`](/docs/planning/test-refactoring-plan.yml) - Test organization

### Research & Analysis

- [`exness-eurusd-variant-analysis.md`](/docs/planning/research/exness-eurusd-variant-analysis.md) - EURUSD variant selection
- [`exness-tick-data-evaluation.md`](/docs/planning/research/exness-tick-data-evaluation.md) - Exness data quality analysis

---

## Reference Documents

### Command References

- [`audit-quick-reference.md`](/docs/planning/current/audit-quick-reference.md) - Repository audit commands
- [`README.md`](/docs/planning/README.md) - Directory structure guide

---

## By Category

### Architecture & Design

- [`polars-integration-plan.yml`](/docs/planning/architecture/polars-integration-plan.yml) - Polars dataframe integration
- [`restructure-v2.3.0-migration.md`](/docs/planning/architecture/restructure-v2.3.0-migration.md) - v2.3.0 restructure

### Research & Analysis

- [`critical-finding-flat-bars-impossible.yml`](/docs/planning/research/critical-finding-flat-bars-impossible.yml) - Mathematical constraints
- [`extended-entropy-analysis.yml`](/docs/planning/research/extended-entropy-analysis.yml) - Market entropy analysis
- [`pattern-continuation-entropy-research-synthesis.yml`](/docs/planning/research/pattern-continuation-entropy-research-synthesis.yml) - Pattern research synthesis
- [`exness-eurusd-variant-analysis.md`](/docs/planning/research/exness-eurusd-variant-analysis.md) - EURUSD variant selection
- [`exness-tick-data-evaluation.md`](/docs/planning/research/exness-tick-data-evaluation.md) - Exness data quality

### Current Work

- [`walk-forward-pattern-recognition-system.md`](/docs/planning/current/walk-forward-pattern-recognition-system.md) - ML system
- [`code-consolidation-plan.yml`](/docs/planning/current/code-consolidation-plan.yml) - DRY implementation
- [`data-structure-validation-plan.yml`](/docs/planning/current/data-structure-validation-plan.yml) - Validation system
- [`historical-replay-visualizer.yml`](/docs/planning/current/historical-replay-visualizer.yml) - Replay system
- [`hybrid-plan-phase1.md`](/docs/planning/current/hybrid-plan-phase1.md) - Phase 1 complete
- [`quick-wins-plan.md`](/docs/planning/current/quick-wins-plan.md) - Quick improvements
- [`repository-audit-plan.md`](/docs/planning/current/repository-audit-plan.md) - Audit planning
- [`audit-quick-reference.md`](/docs/planning/current/audit-quick-reference.md) - Command reference
- [`COMPREHENSIVE-TODO-LIST.md`](/docs/planning/current/COMPREHENSIVE-TODO-LIST.md) - Master tracking
- [`codebase-sanitization-plan.yml`](/docs/planning/current/codebase-sanitization-plan.yml) - Cleanup plan

---

## Document Statistics

| Category     | Active | Completed | Reference | Total  |
| ------------ | ------ | --------- | --------- | ------ |
| Current      | 5      | 1         | 2         | 8      |
| Architecture | 0      | 1         | 1         | 2      |
| Research     | 0      | 2         | 3         | 5      |
| Root         | 1      | 3         | 1         | 5      |
| YAML Files   | 4      | 2         | 1         | 7      |
| **Total**    | **10** | **9**     | **8**     | **27** |

---

## Adding New Plans

When creating new planning documents:

1. **Choose the right directory:**
   - `/current/` - Active development work
   - `/architecture/` - System design specs
   - `/research/` - Analysis and findings

2. **Add metadata header:**

   ```markdown
   # Document Title

   **Version**: 1.0.0
   **Created**: YYYY-MM-DD
   **Status**: [pending|in_progress|completed|active|reference]
   **Supersedes**: [file path or N/A]
   ```

3. **Update this INDEX.md** - Add to appropriate section

---

**Navigation**: [Parent README.md](/docs/planning/README.md) | [Repository Root](/README.md)
