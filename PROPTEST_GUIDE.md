# Property-Based Testing Guide

This project uses [proptest](https://proptest-rs.github.io/proptest/) for property-based testing, complementing traditional unit tests.

## 🤔 What is Property-Based Testing?

Unlike unit tests that verify specific cases, property-based tests verify **properties** that must hold for **all** possible inputs.

### Example Comparison

**Unit Test (traditional):**
```rust
#[test]
fn test_enforce_idempotent_specific() {
    let result1 = Nickname::enforce("hello");
    let result2 = Nickname::enforce("hello");
    assert_eq!(result1, result2);
}
```

**Property-Based Test:**
```rust
proptest! {
    #[test]
    fn enforce_is_idempotent(s in unicode_string()) {
        if let Ok(enforced1) = Nickname::enforce(&s) {
            let enforced2 = Nickname::enforce(enforced1.as_ref())?;
            prop_assert_eq!(enforced1.as_ref(), enforced2.as_ref());
        }
    }
}
```

Proptest generates **1000 random strings** and verifies the property holds for all of them.

## 📊 Project Statistics

### Proptest Coverage

| File | Cases per test |
|------|----------------|
| `proptest_properties.rs` (profiles) | 500-1000 |
| `proptest_stringclass.rs` (core) | 500-5000 |

**Coverage**: Comprehensive property tests across all profiles and core classes
**Total cases**: Thousands of automatically generated inputs per test run

### Overall Coverage
```
Unit + integration tests:  Comprehensive
Property-based tests:       Comprehensive (profiles + core)
Doc tests:                  All examples tested
─────────────────────────────────────
Coverage:                   High (all critical paths)
```

## 🎯 Properties Tested

### 1. Idempotence
**Property**: Applying an operation twice gives the same result as applying it once.

```rust
enforce(enforce(s)) == enforce(s)
prepare(prepare(s)) == prepare(s)
```

**Files**: `proptest_properties.rs`

### 2. Symmetry
**Property**: The order of arguments doesn't matter.

```rust
compare(a, b) == compare(b, a)
```

**Files**: `proptest_properties.rs`

### 3. Reflexivity
**Property**: Comparing something to itself always returns true.

```rust
compare(a, a) == Ok(true)
```

**Files**: `proptest_properties.rs`

### 4. Transitivity
**Property**: If a=b and b=c, then a=c.

```rust
if compare(a,b) == Ok(true) && compare(b,c) == Ok(true)
then compare(a,c) == Ok(true)
```

**Files**: `proptest_properties.rs`

### 5. Determinism
**Property**: Same input always produces same output.

```rust
enforce(s) == enforce(s) // always
```

**Files**: `proptest_properties.rs`, `proptest_stringclass.rs`

### 6. Class Consistency
**Property**: `FreeformClass` accepts everything `IdentifierClass` accepts.

```rust
if IdentifierClass.allows(s) == Ok()
then FreeformClass.allows(s) == Ok()
```

**Files**: `proptest_stringclass.rs`

### 7. Exhaustive Classification
**Property**: Every codepoint has a classification.

```rust
forall cp: get_value_from_codepoint(cp) ∈ {PValid, SpecClassPval, ...}
```

**Files**: `proptest_stringclass.rs`

### 8. Length Preservation
**Property**: Enforce doesn't increase size (only trims).

```rust
len(enforce(s)) <= len(s)
```

**Files**: `proptest_properties.rs`

## 🚀 Running Property Tests

### All property tests:
```bash
# Profiles
cargo test --package precis-profiles --test proptest_properties

# Core
cargo test --package precis-core --test proptest_stringclass

# Both
cargo test proptest
```

### Specific test:
```bash
cargo test --test proptest_properties enforce_is_idempotent
```

### With more cases:
```bash
# Default: 1000 cases
# Increase to 10000:
PROPTEST_CASES=10000 cargo test proptest
```

### With verbose output:
```bash
cargo test proptest -- --nocapture
```

### In CI/CD (GitHub Actions):

Property tests run in CI:

1. **Workflow `rust_checks.yml` (automatic)**: Runs all tests with default configuration (~45,000 proptest cases)
   - Executes automatically on every push/PR to main
   - Fast (~4 seconds)

2. **Workflow `rust_tests.yml` (manual)**: Complete test suite + detailed property tests
   - Executable on-demand from GitHub Actions UI
   - Configurable: 1,000 - 100,000+ cases per test
   - Runs all tests TWICE:
     * First execution: Complete suite with configured proptest
     * Second execution: Property tests with detailed output (`--nocapture`)
   - Useful for exhaustive testing and debugging without commits

To run the manual workflow:
- Go to GitHub Actions → "Property-Based Testing" → "Run workflow"
- Configure number of cases and verbose output
- Review logs for detailed output

See [PROPTEST_CI.md](PROPTEST_CI.md) for complete documentation.

## 🔧 Proptest Configuration

### In tests:
```rust
use proptest::prelude::*;
use proptest::test_runner::FileFailurePersistence;

proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
        cases: 1000,
        .. ProptestConfig::default()
    })]

    #[test]
    fn my_property(s in ascii_string()) {
        // test code
    }
}
```

**Note about FileFailurePersistence:**
- Use `WithSource("proptest-regressions")` for integration tests in `tests/` directory
- Proptest saves failing cases in `proptest-regressions/` for regression testing
- Avoids warning "failed to find lib.rs or main.rs" from default configuration

### Environment Variables:

| Variable | Purpose | Default |
|----------|---------|---------|
| `PROPTEST_CASES` | Number of cases to generate | 256 (or configured) |
| `PROPTEST_MAX_SHRINK_ITERS` | Shrinking attempts | 1024 |
| `PROPTEST_VERBOSE` | Detailed output | false |

## 📝 Generation Strategies

The project defines several strategies for generating inputs:

### ASCII Strings
```rust
fn ascii_string() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9_-]{1,50}"
}
```

### Unicode Strings
```rust
fn unicode_string() -> impl Strategy<Value = String> {
    "[\\PC]{0,100}"  // Any printable Unicode character
}
```

### International Strings
```rust
fn international_string() -> impl Strategy<Value = String> {
    prop_oneof![
        "[a-zA-Z0-9]{1,50}",   // ASCII
        "[α-ωΑ-Ω]{1,30}",      // Greek
        "[а-яА-Я]{1,30}",      // Cyrillic
        "[一-龯]{1,20}",        // CJK
        "[ぁ-ん]{1,30}",        // Hiragana
    ]
}
```

### Unicode Codepoints
```rust
fn unicode_codepoint() -> impl Strategy<Value = u32> {
    0x0000u32..=0x10FFFFu32
}
```

## 🐛 Shrinking (Input Minimization)

When a property test fails, proptest automatically **minimizes** the failing input:

```
Test failed: assertion failed
minimal failing input: s = "ￊ"
	successes: 0
	local rejects: 0
	global rejects: 0
```

This is extremely useful for debugging - instead of a complex 100-character string, you get the minimal input that reproduces the bug.

## 📂 proptest-regressions Directory

Proptest automatically saves failing cases in the `proptest-regressions/` directory:

```
precis-core/tests/proptest-regressions/
  proptest_stringclass/
    stringclass_properties/
      char_codepoint_consistency.txt

precis-profiles/tests/proptest-regressions/
  proptest_properties/
    nickname_properties/
      enforce_is_idempotent.txt
```

### Purpose

1. **Regression Testing**: If a test fails once, proptest saves the failing input
2. **Reproducibility**: Next time you run tests, proptest checks these known cases first
3. **CI/CD**: You can commit these files so CI detects regressions

### Should I commit proptest-regressions/?

**Recommendation: NO** (for this project)

- Tests already provide comprehensive coverage with property tests
- Property tests serve primarily for local edge case discovery
- Committing would add repository noise
- If a real bug is discovered, better to create a specific unit test

If a property test finds a bug:
1. Investigate the minimized case
2. Create a specific unit test for that case
3. Add `.proptest-regressions/` to `.gitignore` (already done)

## 📚 Use Cases in PRECIS

### 1. Validate RFC Invariants
Proptest is perfect for verifying RFC rules are followed:
- Idempotence of enforce
- Transitivity of compare
- Consistency between profiles

### 2. Find Edge Cases
Proptest can find character combinations you hadn't considered:
- Halfwidth Hangul characters
- Emoji + combining characters
- Complex BiDi sequences

### 3. Verify Optimizations
When refactoring for performance, proptest verifies behavior doesn't change:
- CharCache must give same results
- Optimized algorithms maintain properties

## 🎓 Best Practices

### 1. Write Properties, Not Tests
❌ **Bad**: "This specific string must pass"
✅ **Good**: "Any valid string must be idempotent"

### 2. Use Conditionals for Invalid Inputs
```rust
// ✅ Correct
if let Ok(result) = Nickname::enforce(&s) {
    prop_assert!(result.len() <= s.len());
}

// ❌ Incorrect (panics if s is invalid)
let result = Nickname::enforce(&s).unwrap();
```

### 3. Document Properties
```rust
/// Property: compare is symmetric
/// compare(a, b) == compare(b, a)
#[test]
fn compare_is_symmetric(...)
```

### 4. Adjust Number of Cases
```rust
// Fast operations: more cases
#![proptest_config(ProptestConfig::with_cases(5000))]

// Slow operations: fewer cases
#![proptest_config(ProptestConfig::with_cases(100))]
```

## 🔍 Interpreting Failures

### Real Failure Example

During development, proptest found:

```
Test failed: enforce should succeed for non-empty prepared strings: "ￊ"
minimal failing input: s = "ￊ"
```

**Analysis:**
- Character: U+FFCA (Halfwidth Hangul Letter Ieung)
- Problem: The property "prepare OK → enforce OK" was incorrect
- Reason: enforce() does more than prepare() (trimming spaces)
- Solution: Invert the property: "enforce OK → prepare OK"

This is the power of proptest: **it found a bug in the property**, not the code.

## 📊 Proptest Performance

| Test Suite | Tests | Cases | Time |
|------------|-------|-------|------|
| Unit tests | 211 | N/A | ~2-3s |
| Proptest (profiles) | 18 | ~18,000 | ~0.6s |
| Proptest (core) | 20 | ~30,000 | ~1.3s |

Proptest is **extremely efficient**: 50,000 cases in ~2 seconds.

## 🚀 Adding New Property Tests

### 1. Identify a Property
Ask: "What must always be true?"

Examples:
- Idempotent operations
- Symmetric comparisons
- Mathematical invariants
- Relationships between functions

### 2. Write the Test
```rust
proptest! {
    #[test]
    fn my_property(input in my_strategy()) {
        if let Ok(result) = my_function(input) {
            prop_assert!(/* property check */);
        }
    }
}
```

### 3. Define a Strategy
```rust
fn my_strategy() -> impl Strategy<Value = MyType> {
    // Generate valid inputs
    "[a-z]{1,20}"
}
```

### 4. Run and Refine
```bash
cargo test my_property
```

If it fails, proptest gives you the minimal input that reproduces the failure.

## 📚 References

- [Proptest Book](https://proptest-rs.github.io/proptest/)
- [Proptest on crates.io](https://crates.io/crates/proptest)
- [Property-Based Testing (QuickCheck paper)](https://www.cs.tufts.edu/~nr/cs257/archive/john-hughes/quick.pdf)
- [Hypothesis (Python equivalent)](https://hypothesis.readthedocs.io/)

---

**Status**: ✅ 38 property tests implemented, all passing
**Cases generated**: ~30,000-40,000 per complete run
**Time**: ~2 seconds for all property tests
