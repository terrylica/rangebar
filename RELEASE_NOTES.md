
## 2.1.0 - 2025-10-01


### ✨ New Features

- Implement git-cliff for automated changelog and release notes Integrate git-cliff as off-the-shelf solution for dual-output changelog generation: - Install git-cliff v2.10.1 via cargo (Rust-native performance) - Create cliff.toml for detailed CHANGELOG.md (developer-focused) - Create cliff-release-notes.toml for RELEASE_NOTES.md (user-focused) - Add scripts/release.sh for unified release workflow - Update CLAUDE.md with release process documentation Benefits: - Single source of truth: git history → dual outputs - Zero manual changelog maintenance - Consistent formatting with emoji sections - GitHub username attribution - Automated version bumping with Commitizen integration Workflow: ./scripts/release.sh executes: 1. Commitizen version bump 2. git-cliff CHANGELOG.md generation 3. git-cliff RELEASE_NOTES.md generation 4. Git push with tags 5. GitHub release creation Configuration files use TOML array of tables syntax for commit parsers and preprocessors. (by @terrylica)



### 🐛 Bug Fixes & Improvements

- Resolve formatting issues for CI compliance (by @terrylica)



### 📝 Other Changes

- Version 2.0.0 → 2.1.0 (by @terrylica)



---
**Full Changelog**: https://github.com/Eon-Labs/rangebar/compare/v2.0.0...v2.1.0
