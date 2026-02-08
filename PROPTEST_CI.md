# Proptest CI/CD Integration

This document explains how property-based testing with proptest is integrated into the GitHub Actions CI/CD pipeline.

## 🔄 Workflows Running Proptest

### 1. Workflow `rust_checks.yml` - Automatic Testing
```yaml
- name: Run tests
  run: cargo test --verbose
```

**Trigger:** Automatic on every push/PR to main
**Cases executed:** Default configuration in code (500-5000 per test)
- `proptest_properties.rs`: 18 tests × 500-1000 cases ≈ 15,000 cases
- `proptest_stringclass.rs`: 20 tests × 500-5000 cases ≈ 30,000 cases
- Unit tests: 211 tests

**Total:** 249 tests (~45,000 proptest cases)
**Time:** ~4 seconds

### 2. Workflow `rust_tests.yml` - Manual Testing ⭐

```yaml
name: Property-Based Testing
on: workflow_dispatch
```

**Trigger:** Manual from GitHub Actions UI
**What it runs:**
1. **All tests** (249 tests: unit + property + integration + doc)
2. **Property tests with detailed output** (--nocapture)

**Configurable parameters:**
- `cases`: Number of proptest cases (default: 1000)
- `verbose`: Detailed proptest output (default: true)

**Environment variables:**
- `PROPTEST_CASES`: Configurable via input
- `PROPTEST_VERBOSE`: Configurable via input

**Advantages:**
- ✅ Executable on-demand without push
- ✅ Complete test suite with configurable proptest
- ✅ Adjustable from UI (1000, 10000, 100000 cases)
- ✅ Detailed output with `--nocapture` for property tests
- ✅ Doesn't affect main CI pipeline

## 🎯 Proptest Environment Variables

### In Manual Workflow `rust_tests.yml`

```yaml
- name: Run all tests
  env:
    PROPTEST_CASES: ${{ inputs.cases }}        # Configurable (default: 1000)
    PROPTEST_VERBOSE: ${{ inputs.verbose }}    # Configurable (default: true)
  run: cargo test --workspace --verbose
```

**Workflow inputs:**
- `cases`: Number of cases to run (1000, 10000, 100000, etc.)
- `verbose`: Enable detailed output (true/false)

**How to execute manually:**
1. Go to **GitHub Actions** in your repository
2. Select **"Property-Based Testing"** in sidebar
3. Click **"Run workflow"**
4. Configure:
   - **cases**: 1000 (fast), 10000 (moderate), 100000 (exhaustive)
   - **verbose**: true (recommended for details)
5. Click **"Run workflow"** and wait for results

**What does PROPTEST_VERBOSE do?**
- Shows information about running cases
- Useful for debugging
- Displays generation progress

Other available variables (not configured):
- `PROPTEST_MAX_SHRINK_ITERS`: Shrinking attempts (default: 1024)
- `PROPTEST_MAX_FLAT_MAP_REGENS`: Flat_map regenerations (default: 1000000)

## 📂 Regression Files

Proptest regression files are in `.gitignore`:

```gitignore
# Property-based testing regression files
**/proptest-regressions/
```

**What are they?**
- `.txt` files that proptest creates when a test fails
- Contain "seeds" of cases that caused failures
- Created in `precis-core/tests/proptest-regressions/` and `precis-profiles/tests/proptest-regressions/`

**Why are they in .gitignore?**
- Not committed by default (keeps repo clean)
- If a test fails in CI, logs show the minimized input
- Can create specific unit test for important cases

**Note:** If you want to commit regression files for "memory" between CI runs, remove them from `.gitignore`.

## 🔍 Debugging in CI

### Detailed Output

CI uses `PROPTEST_VERBOSE=1` to show detailed information:

```yaml
env:
  PROPTEST_VERBOSE: 1
```

### Verify Cases Executed

In CI logs you'll see:

```
running 249 tests
test nickname_properties::enforce_is_idempotent ... ok
test nickname_properties::compare_is_symmetric ... ok
...
test result: ok. 249 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

With `PROPTEST_VERBOSE=1` you'll also see case generation information.

### If a Test Fails in CI

1. **See minimized input in logs:**
   ```
   Test failed: enforce_is_idempotent
   minimal failing input: s = "ￊ"
   ```

2. **Reproduce locally:**
   ```bash
   PROPTEST_CASES=1000 PROPTEST_VERBOSE=1 cargo test enforce_is_idempotent -- --nocapture
   ```

3. **Create specific unit test:**
   ```rust
   #[test]
   fn test_halfwidth_hangul_regression() {
       let input = "ￊ";  // U+FFCA
       let result = Nickname::enforce(input);
       assert!(result.is_ok());
   }
   ```

## 📊 Execution Statistics

### Workflow `rust_checks.yml` - Automatic

| Job | Tests | Proptest Cases | Time |
|-----|-------|----------------|------|
| `build` | 249 (211 unit + 38 property) | ~45,000 | ~4s |

### Workflow `rust_tests.yml` - Manual

**Runs: All tests (249) + Property tests detailed**

| Configuration | Cases/Test | Total Proptest Cases | Estimated Time |
|---------------|------------|----------------------|----------------|
| Default (1,000) | 1,000 | 38,000 | ~6s (4s tests + 2s detailed proptest) |
| Moderate (10,000) | 10,000 | 380,000 | ~14s (4s tests + 10s detailed proptest) |
| Exhaustive (100,000) | 100,000 | 3,800,000 | ~94s (4s tests + 90s detailed proptest) |

### Property Test Breakdown

| File | Tests | Cases/Test (manual) | Total Cases | Time |
|------|-------|---------------------|-------------|------|
| `proptest_properties.rs` | 18 | Configurable | 18 × cases | ~0.7-7s |
| `proptest_stringclass.rs` | 20 | Configurable | 20 × cases | ~1.3-13s |
| **TOTAL** | **38** | **Configurable** | **38 × cases** | **~2-20s** |

### Total Coverage

```
Unit tests:              211 tests
Property-based tests:     38 tests
Doc tests:                 7 tests
────────────────────────────────
TOTAL:                   256 tests
```

## 🛠️ Configuration Best Practices

### For Local Development

Keep configurations low for speed:

```rust
proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
        cases: 1000,  // ← Fast for development
        .. ProptestConfig::default()
    })]
}
```

### For CI

Use `PROPTEST_CASES` to override:

```yaml
env:
  PROPTEST_CASES: 1000  # ← CI override
```

### For Bug Investigation

Temporarily increase cases:

```bash
PROPTEST_CASES=100000 cargo test proptest
```

## ✅ Verification

### Commands to Test Locally

```bash
# Simulate automatic workflow (rust_checks.yml)
cargo test --verbose

# Simulate complete manual workflow (rust_tests.yml)
# Step 1: All tests
PROPTEST_CASES=1000 PROPTEST_VERBOSE=1 cargo test --workspace --verbose

# Step 2: Property tests with detailed output
PROPTEST_CASES=1000 PROPTEST_VERBOSE=1 cargo test --test proptest_properties --verbose -- --nocapture
PROPTEST_CASES=1000 PROPTEST_VERBOSE=1 cargo test --test proptest_stringclass --verbose -- --nocapture

# Simulate with more cases (exhaustive)
PROPTEST_CASES=10000 PROPTEST_VERBOSE=1 cargo test --workspace --verbose

# Check for regressions
find . -name "proptest-regressions" -type d
```

### Expected State in CI

**Workflow `rust_checks.yml` (automatic):**
✅ All tests pass (249 tests)
✅ Proptest runs ~45,000 cases without failures
✅ Completes in ~4 seconds

**Workflow `rust_tests.yml` (manual):**
✅ **First execution:** All tests pass (249 tests) with configurable proptest
✅ **Second execution:** Property tests with detailed output (`--nocapture`)
✅ No regression files (git-ignored)
✅ Time varies by configuration (6-94s)
✅ Total proptest cases executed: cases × 38 tests

## 🚀 How to Run the Manual Workflow

1. Go to your repository on GitHub
2. Click on the **Actions** tab
3. In the left sidebar, select **"Property-Based Testing"**
4. Click the **"Run workflow"** button (right side)
5. Configure parameters:
   - **cases**: 1000 (default), 10000, 100000
   - **verbose**: true (default) or false
6. Click **"Run workflow"**
7. Wait for completion and review logs

**What will it execute?**
1. ✅ Complete test suite (249 tests) with configured proptest
2. ✅ Property tests again with detailed output (`--nocapture`)
3. ✅ Summary with execution statistics

**Workflow dispatch UI:**
```
┌─────────────────────────────────────────┐
│ Run workflow                             │
├─────────────────────────────────────────┤
│ Use workflow from: main                  │
│                                          │
│ Number of test cases per property test  │
│ [1000                               ▼]  │
│                                          │
│ Enable verbose output                    │
│ [✓] true                                │
│                                          │
│          [Run workflow]                  │
└─────────────────────────────────────────┘
```

## 📊 Workflow Comparison

| Workflow | Trigger | Tests | Proptest Cases | Time | Purpose |
|----------|---------|-------|----------------|------|---------|
| `rust_checks.yml` | Automatic (push/PR) | 249 | ~45k (default) | ~4s | Fast CI |
| `rust_tests.yml` | Manual (on-demand) | 249 × 2 | Configurable | 6-94s | Exhaustive testing + debugging |

**Key differences:**
- **`rust_checks.yml`**: One execution with defaults, focus on speed for CI
- **`rust_tests.yml`**: Two executions (normal + detailed), focus on exhaustiveness and debugging

## 🔬 Future Improvements

Possible enhancements to consider:

1. **Proptest Nightly:**
   ```yaml
   - name: Run proptest with nightly
     run: cargo +nightly test --test proptest_properties
   ```

2. **Parallel Execution:**
   ```yaml
   - name: Install cargo-nextest
     run: cargo install cargo-nextest
   - name: Run proptest with nextest
     run: cargo nextest run --test proptest_properties
   ```

3. **Different Seeds:**
   ```yaml
   - name: Run proptest with different seeds
     run: |
       for i in {1..5}; do
         PROPTEST_RNG_SEED=$RANDOM cargo test proptest
       done
   ```

4. **Upload Regression Files (optional):**
   ```yaml
   - name: Upload regression files
     if: failure()
     uses: actions/upload-artifact@v4
     with:
       name: proptest-regressions
       path: '**/proptest-regressions/'
   ```

---

**Last updated:** 2026-02-07
**Property tests:** 38 (18 profiles + 20 core)
**Cases in automatic CI:** ~45,000 per run
**Cases in manual CI:** Configurable (1,000 - 100,000+)
