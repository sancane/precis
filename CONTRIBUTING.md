# Contributing to PRECIS

Thank you for your interest in contributing to the Rust PRECIS Framework library! This document provides guidelines and information to help you contribute effectively.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Types of Contributions](#types-of-contributions)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Benchmarking](#benchmarking)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Commit Messages](#commit-messages)
- [License](#license)

## Code of Conduct

Be respectful, considerate, and professional. We welcome contributions from everyone who shares these values.

## Getting Started

### Prerequisites

- **Rust**: 1.80+ (for stable `LazyLock` support)
- **Cargo**: Latest stable
- **Git**: For version control

### Setup Development Environment

1. **Fork and clone the repository:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/precis.git
   cd precis
   ```

2. **Build the project:**
   ```bash
   cargo build --workspace
   ```

3. **Run tests to verify setup:**
   ```bash
   cargo test --workspace
   ```

4. **Install development tools:**
   ```bash
   # Formatter
   rustup component add rustfmt

   # Linter
   rustup component add clippy

   # Cargo.toml formatter
   cargo install cargo-sort

   # Coverage tool (optional)
   cargo install cargo-tarpaulin

   # Benchmark tool (optional)
   cargo install cargo-codspeed
   ```

## Project Structure

The project is organized as a Cargo workspace with three crates:

```
precis/
├── precis-core/         # Core PRECIS Framework (RFC 8264)
│   ├── src/            # Base string classes
│   ├── tests/          # Integration tests
│   └── benches/        # Performance benchmarks
├── precis-profiles/    # PRECIS Profiles (RFC 8265, 8266)
│   ├── src/            # Username, Password, Nickname profiles
│   ├── tests/          # Profile tests + property-based tests
│   └── benches/        # Profile benchmarks
└── precis-tools/       # Build tools (UCD parsers)
    └── src/            # Code generation tools
```

### Key Components

- **precis-core**: Implements `IdentifierClass` and `FreeformClass` base string classes
- **precis-profiles**: Implements `UsernameCaseMapped`, `UsernameCasePreserved`, `OpaqueString`, and `Nickname` profiles
- **precis-tools**: Build-time tools for generating Unicode character tables from UCD data

## Types of Contributions

### 🐛 Bug Fixes

Found a bug? Please:
1. Check if it's already reported in [Issues](https://github.com/sancane/precis/issues)
2. Create a new issue if not
3. Include a minimal reproducible example
4. Submit a PR with a fix and test case

### ✨ New Features

Before implementing a new feature:
1. Open an issue to discuss the proposal
2. Ensure it aligns with RFC specifications
3. Consider backward compatibility
4. Update documentation

### 📝 Documentation

Documentation improvements are always welcome:
- Fix typos or unclear explanations
- Add examples
- Improve API documentation
- Translate documentation (currently English)

### 🧪 Testing

Help improve test coverage:
- Add edge case tests
- Write property-based tests
- Add RFC test vectors
- Improve existing test clarity

### ⚡ Performance

Performance improvements:
1. Add benchmark first to establish baseline
2. Implement optimization
3. Verify no behavioral changes
4. Document performance impact

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

**Branch naming conventions:**
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation changes
- `perf/` - Performance improvements
- `test/` - Test additions/improvements

### 2. Make Changes

- Write clean, idiomatic Rust code
- Follow existing code style
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test test_name

# Run with property-based testing
PROPTEST_CASES=10000 cargo test proptest

# Check formatting
cargo fmt --all -- --check

# Check Cargo.toml formatting
cargo sort -w --check

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Run benchmarks (if applicable)
cargo bench
```

### 4. Update Documentation

```bash
# Build documentation locally
cargo doc --workspace --no-deps --open

# Check doc tests
cargo test --doc
```

## Coding Standards

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Use `cargo sort` to keep dependencies organized
- Address all `clippy` warnings
- Use meaningful variable and function names
- Prefer clarity over cleverness

### Code Quality

```bash
# Format code
cargo fmt --all

# Sort Cargo.toml dependencies
cargo sort -w

# Check for common mistakes
cargo clippy --all-targets --all-features -- -D warnings

# Check for unused dependencies
cargo +nightly udeps --all-targets
```

### Error Handling

- Use `Result<T, E>` for recoverable errors
- Use `Option<T>` for optional values
- Provide meaningful error messages
- Document error conditions

### Comments and Documentation

- Add doc comments (`///`) for public APIs
- Include examples in doc comments
- Explain "why", not "what" in code comments
- Reference RFC sections when applicable

Example:
```rust
/// Enforces the PRECIS Nickname profile on the input string.
///
/// This function applies the Nickname profile as defined in RFC 8266,
/// including Unicode normalization, width mapping, and trimming.
///
/// # Examples
///
/// ```
/// use precis_profiles::Nickname;
/// use precis_core::profile::PrecisFastInvocation;
///
/// let result = Nickname::enforce("   Alice   ");
/// assert_eq!(result, Ok(std::borrow::Cow::from("Alice")));
/// ```
///
/// # Errors
///
/// Returns an error if the input contains disallowed characters
/// or violates BiDi rules (RFC 5893).
pub fn enforce(s: &str) -> Result<Cow<'_, str>, Error> {
    // Implementation
}
```

## Testing

### Test Coverage Goal

The project maintains high test coverage. All contributions should include appropriate tests.

### Types of Tests

#### 1. Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enforce_ascii() {
        let result = Nickname::enforce("alice");
        assert_eq!(result, Ok(Cow::from("alice")));
    }
}
```

#### 2. Integration Tests

Place integration tests in `tests/` directory:

```rust
// tests/nickname_tests.rs
use precis_profiles::Nickname;
use precis_core::profile::PrecisFastInvocation;

#[test]
fn test_nickname_unicode() {
    assert_eq!(
        Nickname::enforce("Иван"),
        Ok(std::borrow::Cow::from("Иван"))
    );
}
```

#### 3. Property-Based Tests

Add property tests for algorithmic properties:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn enforce_is_idempotent(s in "[\\PC]{0,100}") {
        if let Ok(enforced1) = Nickname::enforce(&s) {
            let enforced2 = Nickname::enforce(enforced1.as_ref())?;
            prop_assert_eq!(enforced1.as_ref(), enforced2.as_ref());
        }
    }
}
```

See [PROPTEST_GUIDE.md](PROPTEST_GUIDE.md) for complete property-based testing guide.

#### 4. RFC Test Vectors

Verify compliance with RFC test vectors:

```rust
#[test]
fn test_rfc8266_vectors() {
    // Test vectors from RFC 8266 Appendix
    assert!(Nickname::enforce("juliet@example.com").is_ok());
    // ... more vectors
}
```

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test --package precis-core

# Specific test
cargo test test_nickname_enforcement

# Property tests with more cases
PROPTEST_CASES=10000 cargo test proptest

# Show test output
cargo test -- --nocapture

# Generate coverage report
cargo tarpaulin --workspace --exclude precis-tools --timeout 120 --out Html
```

## Benchmarking

### When to Add Benchmarks

Add benchmarks for:
- New profile implementations
- Performance-critical functions
- Optimizations

### Benchmark Guidelines

Place benchmarks in `benches/` directory:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use precis_profiles::Nickname;
use precis_core::profile::PrecisFastInvocation;

fn bench_nickname_enforce(c: &mut Criterion) {
    c.bench_function("Nickname::enforce ASCII", |b| {
        b.iter(|| Nickname::enforce(black_box("alice")))
    });
}

criterion_group!(benches, bench_nickname_enforce);
criterion_main!(benches);
```

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench profiles_bench

# Compare with baseline
cargo bench -- --save-baseline main
# ... make changes ...
cargo bench -- --baseline main
```

See [BENCHMARKING.md](BENCHMARKING.md) for complete benchmarking guide.

## Documentation

### Documentation Requirements

All public APIs must have:
- Summary description
- Parameter descriptions
- Return value description
- Error conditions
- At least one example
- Links to relevant RFCs

### Building Documentation

```bash
# Build and open docs
cargo doc --workspace --no-deps --open

# Check for broken links
cargo doc --workspace --no-deps 2>&1 | grep warning

# Test doc examples
cargo test --doc
```

## Pull Request Process

### Before Submitting

Ensure your PR:
- ✅ Builds successfully: `cargo build --workspace`
- ✅ Passes all tests: `cargo test --workspace`
- ✅ Passes clippy: `cargo clippy --all-targets --all-features -- -D warnings`
- ✅ Is properly formatted: `cargo fmt --all -- --check`
- ✅ Has sorted dependencies: `cargo sort -w --check`
- ✅ Includes tests for new functionality
- ✅ Updates relevant documentation
- ✅ Maintains or improves test coverage

**Quick check script:**
```bash
# Run all checks at once
cargo build --workspace && \
cargo test --workspace && \
cargo fmt --all -- --check && \
cargo sort -w --check && \
cargo clippy --all-targets --all-features -- -D warnings
```

**Fix formatting issues:**
```bash
# Auto-fix formatting
cargo fmt --all
cargo sort -w
```

### Submitting the PR

1. **Push your branch:**
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create Pull Request on GitHub:**
   - Provide a clear title
   - Describe what changes were made
   - Reference related issues
   - Include test results if applicable

3. **PR Template:**
   ```markdown
   ## Description
   Brief description of changes

   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Performance improvement
   - [ ] Documentation update

   ## Testing
   - [ ] Added/updated unit tests
   - [ ] Added/updated integration tests
   - [ ] Added/updated property-based tests
   - [ ] All tests pass

   ## Checklist
   - [ ] Code follows project style guidelines
   - [ ] Self-review completed
   - [ ] Documentation updated
   - [ ] No new warnings from clippy
   - [ ] Benchmark results included (if applicable)

   ## Related Issues
   Fixes #123
   ```

### CI/CD Checks

Your PR will automatically run:
- **Build and Test** (`rust_checks.yml`): Compiles and runs all tests
- **Coverage** (`coverage.yml`): Verifies test coverage doesn't decrease
- **Security Audit** (`security_audit.yml`): Checks for known vulnerabilities
- **Benchmarks** (`benchmarks.yml`): Tracks performance changes with CodSpeed

All checks must pass before merging.

### Review Process

1. Maintainers will review your PR
2. Address any requested changes
3. Once approved, a maintainer will merge

**Response time:** We aim to review PRs within 1 week.

## Commit Messages

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

### Examples

```
feat(profiles): add Nickname profile implementation

Implement Nickname profile as defined in RFC 8266, including
width mapping, space trimming, and case folding.

Closes #42
```

```
fix(core): correct BiDi rule validation for NSM sequences

Fixed bug where multiple NSM characters at string boundaries
were incorrectly rejected. Added test case to verify fix.

Fixes #87
```

```
perf(profiles): optimize username normalization

Reduced allocations by using Cow::Borrowed when no changes
are needed. Benchmark shows 15% improvement for ASCII inputs.

Benchmark results:
- Before: 2.3µs
- After:  2.0µs (-13%)
```

## License

By contributing, you agree that your contributions will be dual-licensed under:
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](https://opensource.org/licenses/MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you shall be dual licensed as above, without any additional terms or conditions.

## Questions?

If you have questions:
- Open a [Discussion](https://github.com/sancane/precis/discussions) on GitHub
- Open an [Issue](https://github.com/sancane/precis/issues) for bugs or feature requests
- Check existing documentation:
  - [README.md](README.md) - Project overview
  - [PROPTEST_GUIDE.md](PROPTEST_GUIDE.md) - Property-based testing guide
  - [BENCHMARKING.md](BENCHMARKING.md) - Benchmarking guide
  - [CODSPEED.md](CODSPEED.md) - CodSpeed integration

## Recognition

Contributors are recognized in:
- Git commit history
- Release notes (when applicable)
- Project README

Thank you for contributing to the PRECIS Framework! 🎉
