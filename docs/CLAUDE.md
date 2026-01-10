# Documentation Directory

Context for working with project documentation.

**Parent**: [`/CLAUDE.md`](/CLAUDE.md)

## Structure

```
docs/
├── ARCHITECTURE.md          # Crate structure, data flow, public APIs
├── API.md                   # REST API reference (future)
├── specifications/          # Authoritative specs (SSoT)
│   └── algorithm-spec.md    # Core algorithm definition
├── guides/                  # How-to guides
│   ├── publishing.md        # crates.io release workflow
│   ├── common-workflows.md  # Typical usage patterns
│   ├── error-recovery.md    # Error handling strategies
│   └── performance-profiling.md
├── development/             # Developer docs
│   ├── compilation-optimization.md  # Build acceleration
│   ├── automated-updates.md # Dependency update automation
│   ├── MIGRATION-v4-to-v5.md
│   └── disk-cleanup-workflow.md
├── planning/                # Plans and roadmaps
│   ├── INDEX.md             # Navigation hub for all plans
│   ├── current/             # Active work
│   ├── architecture/        # System design
│   ├── research/            # Analysis findings
│   └── legacy/              # Historical/superseded
├── diagrams/                # ASCII and visual diagrams
│   └── INDEX.md             # Diagram catalog
├── reports/                 # Generated reports
├── archive/                 # Historical analysis (Sept 2025)
└── testing/                 # Test documentation
```

## Key Documents

| Document | Purpose | Status |
|----------|---------|--------|
| [`specifications/algorithm-spec.md`](/docs/specifications/algorithm-spec.md) | Canonical algorithm definition | **SSoT** |
| [`ARCHITECTURE.md`](/docs/ARCHITECTURE.md) | System architecture | Current |
| [`guides/publishing.md`](/docs/guides/publishing.md) | Release workflow | Current |
| [`planning/INDEX.md`](/docs/planning/INDEX.md) | All planning docs | Index |

## Document Categories

### Specifications (`specifications/`)

- **SSoT documents** - authoritative, other docs must conform
- Version-controlled with explicit supersedes chain
- Algorithm spec is the canonical reference

### Guides (`guides/`)

- Practical how-to documentation
- Task-oriented, step-by-step
- Examples and commands included

### Development (`development/`)

- Developer-focused documentation
- Build, test, migration guides
- Automation and tooling

### Planning (`planning/`)

- Structured by status: `current/`, `legacy/`, `research/`, `architecture/`
- `INDEX.md` is the navigation hub
- YAML files for structured specs, MD for prose

### Archive (`archive/`)

- Historical reports from Sept 2025
- Adversarial testing, security audits, benchmarks
- Reference only, not maintained

## Writing Guidelines

### New Documents

1. Choose correct directory by purpose
2. Add metadata header:
   ```markdown
   # Title

   **Version**: 1.0.0
   **Status**: [draft|current|archived]
   **Supersedes**: [path or N/A]
   ```
3. Update relevant INDEX.md files
4. Use repo-root links: `[text](/docs/path/file.md)`

### Linking Convention

- Internal: `[Guide](/docs/guides/publishing.md)` (repo-root `/`)
- External: Full URL with `https://`
- Relative only within same directory tree

### Diagrams

- ASCII box-drawing for markdown compatibility
- Store in `diagrams/` with INDEX.md entry
- Use `graph-easy` for generation (see global CLAUDE.md skills)
