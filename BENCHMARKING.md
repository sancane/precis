# Benchmarking with Criterion

This project uses [Criterion.rs](https://github.com/bheisler/criterion.rs) for performance benchmarking.

## 🚀 CodSpeed Integration

The project is integrated with [CodSpeed](https://codspeed.io/) for continuous benchmarking in CI/CD:

- **Local development**: Uses Criterion natively (HTML reports)
- **CI/CD**: Uses CodSpeed (historical performance tracking)
- **No code changes**: Integration uses `codspeed-criterion-compat` as a drop-in replacement

**Configuration in Cargo.toml:**
```toml
[dev-dependencies]
# Renamed to codspeed-criterion-compat for CI, no code changes needed
criterion = { package = "codspeed-criterion-compat", version = "*" }
```

**Note:** Using `version = "*"` as recommended by CodSpeed to always use the latest compatible version.

**Advantages:**
- ✅ Automatic performance tracking on every PR
- ✅ Regression detection
- ✅ Historical performance trends visualization
- ✅ No changes to benchmark code

## 📊 Available Benchmarks

### precis-core: `stringclass_bench`

Measures performance of `StringClass` operations:

- **`get_value_from_char`**: Individual character classification
- **`get_value_from_codepoint`**: Classification by Unicode codepoint
- **`allows`**: Complete string validation
- **`allows_by_length`**: Validation by string length

**Test cases:**
- ASCII (letters, digits)
- Unicode (Greek, Cyrillic, CJK)
- Special characters (spaces, ZWNJ)

### precis-profiles: `profiles_bench`

Measures performance of PRECIS profiles:

- **`Nickname`**: enforce, prepare, compare
- **`UsernameCaseMapped`**: enforce, compare (case-insensitive)
- **`UsernameCasePreserved`**: enforce, compare (case-sensitive)
- **`OpaqueString`**: enforce, compare (passwords)

**Additional test cases:**
- **`enforce_by_length`**: Performance by length (10-500 chars)
- **`unicode_complexity`**: Performance with different Unicode scripts

## 🚀 Running Benchmarks

### All benchmarks:
```bash
cargo bench
```

### Specific benchmark:
```bash
# StringClass only
cargo bench --bench stringclass_bench

# Profiles only
cargo bench --bench profiles_bench
```

### Specific function benchmark:
```bash
# Only Nickname benchmarks
cargo bench --bench profiles_bench Nickname

# Only enforce for Nickname
cargo bench --bench profiles_bench "Nickname/enforce"

# Only allows benchmarks
cargo bench --bench stringclass_bench allows
```

### Compare with previous baseline:
```bash
# Save current baseline
cargo bench -- --save-baseline main

# Make changes to code...

# Compare with baseline
cargo bench -- --baseline main
```

## 📈 Interpreting Results

### Criterion Output

```
Nickname/enforce/ASCII simple
                        time:   [1.2345 µs 1.2456 µs 1.2567 µs]
                        change: [-2.34% -1.23% +0.12%] (p = 0.15 > 0.05)
                        No change in performance detected.
```

**Interpretation:**
- **time**: Average execution time (with confidence intervals)
- **change**: Change from previous run (±%)
- **p-value**: Statistical significance (< 0.05 = significant change)

### Time Units

| Unit | Abbreviation | Equivalent |
|------|--------------|------------|
| Nanosecond | ns | 0.000000001 s |
| Microsecond | µs | 0.000001 s |
| Millisecond | ms | 0.001 s |
| Second | s | 1 s |

### Throughput

Some benchmarks show throughput (elements/second):

```
Nickname/enforce/ASCII simple
                        time:   [1.2345 µs 1.2456 µs 1.2567 µs]
                        thrpt:  [802.34 K elem/s 810.45 K elem/s 818.56 K elem/s]
```

## 📂 Result Files

Criterion saves results in `target/criterion/`:

```
target/criterion/
├── Nickname/
│   ├── enforce/
│   │   ├── ASCII simple/
│   │   │   ├── base/
│   │   │   │   └── estimates.json
│   │   │   ├── new/
│   │   │   └── report/
│   │   │       └── index.html    ← HTML report
│   └── report/
│       └── index.html            ← Benchmark index
```

### View HTML reports:

```bash
# Open report of all benchmarks
open target/criterion/report/index.html

# Specific Nickname report
open target/criterion/Nickname/report/index.html
```

## 🎯 Use Cases

### 1. Verify Performance of Optimization

```bash
# Before optimization
cargo bench -- --save-baseline before

# Apply optimization...

# After optimization
cargo bench -- --baseline before
```

Criterion will show if there's significant improvement/regression.

### 2. Compare Profiles

```bash
cargo bench --bench profiles_bench
```

Allows you to see which profile is fastest for different cases.

### 3. Detect Regressions

```bash
# In CI or locally
cargo bench
```

If there are significant changes (> 5%), criterion will report them.

### 4. Profile Long Strings

```bash
cargo bench --bench profiles_bench "enforce_by_length"
```

Verifies if performance is O(n) with respect to length.

## 📊 Benchmarks in CI with CodSpeed

The project uses [CodSpeed](https://codspeed.io/) for continuous benchmarking in CI/CD. See [CODSPEED.md](CODSPEED.md) for complete details.

### Configuration

**1. Cargo.toml uses codspeed-criterion-compat:**

```toml
[dev-dependencies]
criterion = { package = "codspeed-criterion-compat", version = "*" }
```

**2. Automatic workflow:** `.github/workflows/benchmarks.yml`

Runs on:
- Push to main (updates baseline)
- Pull requests (detects regressions)
- Manual dispatch (backtesting)

**3. Results:**

- Dashboard: `https://codspeed.io/[username]/precis`
- Automatic comments on PRs with results
- Regression detection

### Initial Setup

1. Connect your repo at [CodSpeed.io](https://codspeed.io/)
2. Authorize GitHub access
3. Done! No tokens needed (OIDC authentication)

### Tools

- **Local:** `cargo bench` (Criterion native with HTML reports)
- **CI:** `cargo codspeed` (optimized for CodSpeed)

```bash
# Install cargo-codspeed locally (optional)
cargo install cargo-codspeed

# Build benchmarks
cargo codspeed build

# Run with CodSpeed instrumentation
cargo codspeed run
```

See [CODSPEED.md](CODSPEED.md) for complete setup guide and troubleshooting.

## 🔧 Advanced Configuration

### Change Number of Iterations

By default, Criterion decides automatically. To force:

```rust
// In the benchmark .rs file
use criterion::Criterion;
use std::time::Duration;

fn custom_criterion() -> Criterion {
    Criterion::default()
        .sample_size(100)           // Number of samples
        .measurement_time(Duration::from_secs(10))  // Total time
}

criterion_group! {
    name = benches;
    config = custom_criterion();
    targets = bench_nickname
}
```

### Custom Warmup

```rust
fn custom_criterion() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(3))
}
```

### Noise Threshold

```rust
fn custom_criterion() -> Criterion {
    Criterion::default()
        .noise_threshold(0.05)  // 5% allowed noise
}
```

## 🐛 Troubleshooting

### "Cannot compile benchmarks"

Verify that `criterion` is in `[dev-dependencies]` and benchmark has:

```toml
[[bench]]
name = "my_bench"
harness = false
```

### Inconsistent Results

```bash
# Disable CPU frequency scaling
sudo cpupower frequency-set --governor performance

# Close other applications
# Run benchmarks
cargo bench

# Restore governor
sudo cpupower frequency-set --governor powersave
```

### Benchmarks Too Slow

```bash
# Reduce sample size
cargo bench -- --sample-size 10
```

## 📚 References

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Criterion.rs User Guide](https://bheisler.github.io/criterion.rs/book/user_guide/user_guide.html)
- [Cargo Bench Documentation](https://doc.rust-lang.org/cargo/commands/cargo-bench.html)
- [CodSpeed Documentation](https://docs.codspeed.io/)

## 🎯 Expected Metrics

### Typical performance (on a modern laptop):

| Operation | Input | Approx. Time |
|-----------|-------|--------------|
| `StringClass::allows()` | Short ASCII (10 chars) | ~500 ns |
| `StringClass::allows()` | Unicode (50 chars) | ~2 µs |
| `Nickname::enforce()` | Simple ASCII | ~1.5 µs |
| `Nickname::enforce()` | Unicode + spaces | ~3 µs |
| `UsernameCaseMapped::enforce()` | ASCII | ~2 µs |
| `OpaqueString::enforce()` | Typical password | ~2.5 µs |

**Note**: These are estimates. Run `cargo bench` for precise metrics on your system.

---

**Last updated:** 2026-02-07
**Benchmarks:** 2 files (stringclass_bench, profiles_bench)
**Total tests:** ~100+ combinations
