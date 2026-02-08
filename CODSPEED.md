# CodSpeed Integration

This document explains how CodSpeed is integrated for continuous benchmarking in CI/CD.

## 🎯 What is CodSpeed?

[CodSpeed](https://codspeed.io/) is a performance tracking platform that:
- ✅ Runs benchmarks in CI/CD
- ✅ Automatically detects performance regressions
- ✅ Visualizes historical trends
- ✅ Comments on PRs with results

## 🔧 Integration

### 0. Differences Between Local and CI

| Command | Environment | Purpose | Output |
|---------|-------------|---------|--------|
| `cargo bench` | Local | Development and debugging | HTML reports in `target/criterion/` |
| `cargo codspeed run` | Local/CI | Performance tracking | Sends data to CodSpeed dashboard |

**Both use the same benchmark code** thanks to `codspeed-criterion-compat`.

### 1. Cargo.toml

Use `codspeed-criterion-compat` as a drop-in replacement for Criterion:

```toml
[dev-dependencies]
# Renamed: code continues using 'criterion' but uses CodSpeed in CI
criterion = { package = "codspeed-criterion-compat", version = "*" }
```

**Advantages:**
- No changes to Rust code
- Compatible with local Criterion
- CodSpeed in CI automatically

### 2. GitHub Actions Workflow

**File:** `.github/workflows/benchmarks.yml`

```yaml
name: CodSpeed Benchmarks

on:
  push:
    branches: ["main"]
  pull_request:
  workflow_dispatch:

permissions:
  contents: read
  id-token: write  # Required for OIDC authentication

jobs:
  benchmarks:
    name: Run benchmarks
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup rust toolchain, cache and cargo-codspeed binary
        uses: moonrepo/setup-rust@v1
        with:
          channel: stable
          cache-target: release
          bins: cargo-codspeed

      - name: Build the benchmark target(s)
        run: cargo codspeed build

      - name: Run the benchmarks
        uses: CodSpeedHQ/action@v4
        with:
          mode: simulation
          run: cargo codspeed run
```

**Advantages of this approach:**
- ✅ Uses OIDC authentication (no secret tokens needed)
- ✅ `cargo-codspeed` optimized for Rust
- ✅ `simulation` mode for consistent results
- ✅ Integrated cache with `moonrepo/setup-rust`

### 3. Configuration (No Secrets Required)

**You don't need to configure tokens** thanks to OIDC authentication:

1. Go to [CodSpeed.io](https://codspeed.io/)
2. Connect your GitHub repository
3. Authorize access
4. Done! The workflow will work automatically

**OIDC (OpenID Connect)** allows GitHub Actions to authenticate with CodSpeed without requiring secret tokens.

## 📊 How It Works

### In Local Development

```bash
cargo bench
```

- Uses native Criterion
- Generates HTML reports in `target/criterion/`
- Detailed terminal output

### In CI/CD

When running in GitHub Actions:

**Workflow steps:**
1. `cargo codspeed build` - Compiles benchmarks with instrumentation
2. `cargo codspeed run` - Runs in simulation mode
3. CodSpeed action collects metrics and sends them to the dashboard

**Results:**

1. **Push to main:**
   - CodSpeed runs benchmarks
   - Saves results as baseline
   - Updates trend graphs

2. **Pull Request:**
   - CodSpeed runs benchmarks
   - Compares with main baseline
   - Comments on PR with results:
     - ✅ No regressions
     - ⚠️ Regressions detected (% degradation)
     - 🚀 Performance improvements

**Simulation Mode:**
- Runs benchmarks in a virtualized environment
- Results more consistent than real-time
- Eliminates CI hardware variability

## 📈 Dashboard

Once configured, access the dashboard at:

```
https://codspeed.io/[your-username]/precis
```

**Available information:**
- Historical performance graphs
- Comparisons between branches
- Detected regressions
- Trends per benchmark

## 🔍 Example PR Comment

CodSpeed will automatically add comments like:

```
⚡ CodSpeed Performance Report

Performance changes detected:

Benchmark | Change | Status
----------|--------|-------
stringclass/allows/ASCII simple | +2.3% | ⚠️ Slower
profiles/Nickname/enforce | -1.5% | 🚀 Faster
profiles/OpaqueString/compare | ±0% | ✅ No change

Overall: 1 improvement, 1 regression, 38 unchanged
```

## 🛠️ Troubleshooting

### Workflow doesn't run

**Problem:** The `benchmarks.yml` workflow doesn't appear in Actions

**Solution:**
1. Verify the file is in `.github/workflows/`
2. Push to main or create a PR
3. Verify there are no YAML syntax errors

### OIDC Authentication Fails

**Error:** `Error: OIDC authentication failed`

**Solution:**
1. Verify the repository is connected on CodSpeed.io
2. Verify that `permissions: id-token: write` is in the workflow
3. Public repositories: OIDC works automatically
4. Private repositories: Verify permissions in CodSpeed dashboard

### Benchmarks fail in CI

**Error:** Benchmarks pass locally but fail in CI

**Possible causes:**
1. **Timeout:** Increase timeout in the workflow
2. **Memory:** Reduce sample_size in benchmarks
3. **Dependencies:** Verify all dependencies are available

**Solution:**
```yaml
- name: Run benchmarks
  timeout-minutes: 30  # Increase timeout
  uses: CodSpeedHQ/action@v2
```

### Reports don't appear

**Problem:** Benchmarks run but there are no PR comments

**Solution:**
1. Verify token permissions
2. Wait a few minutes (processing may take time)
3. Check the CodSpeed dashboard

## 🚀 Best Practices

### 1. Run locally first

Before creating a PR, run locally to detect regressions:

**Option A: With Criterion (HTML reports):**
```bash
# Save baseline
cargo bench -- --save-baseline main

# Make changes...

# Compare
cargo bench -- --baseline main
```

**Option B: With CodSpeed (simulates CI):**
```bash
# Install cargo-codspeed
cargo install cargo-codspeed

# Build and run
cargo codspeed build
cargo codspeed run
```

### 2. Focus on stable benchmarks

Benchmarks with high variability (outliers) can cause false positives:

```rust
fn custom_criterion() -> Criterion {
    Criterion::default()
        .noise_threshold(0.05)  // 5% noise threshold
        .sample_size(100)       // More samples = more stable
}
```

### 3. Document performance changes

If a PR intentionally changes performance, document it:

```markdown
## Performance Impact

- `Nickname::enforce`: +10% due to new BiDi validation
- Justified by bug fix #123
```

### 4. Configure thresholds

In the CodSpeed dashboard, configure alert thresholds:
- Warning: > 5% regression
- Critical: > 10% regression

## 📚 References

- [CodSpeed Documentation](https://docs.codspeed.io/)
- [codspeed-criterion-compat](https://github.com/CodSpeedHQ/codspeed-rust)
- [GitHub Action](https://github.com/CodSpeedHQ/action)
- [Criterion.rs](https://bheisler.github.io/criterion.rs/book/)

---

**Status:** ✅ Configured
**Benchmarks:** 2 files (stringclass_bench, profiles_bench)
**Total benchmarks:** ~100+ combinations
**Last updated:** 2026-02-07
