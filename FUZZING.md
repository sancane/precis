# Fuzzing Guide

This document explains how to use fuzzing to discover bugs, panics, and edge cases in the PRECIS implementation.

## What is Fuzzing?

Fuzzing is an automated testing technique that provides random or semi-random data to functions to discover:
- Panics and crashes
- Assertion failures
- Undefined behavior
- Edge cases not covered by unit tests
- Performance issues (slow inputs)

## Prerequisites

**Requirements:**
- Rust nightly toolchain
- cargo-fuzz installed

**Installation:**
```bash
# Install nightly toolchain (if not already installed)
rustup install nightly

# Install cargo-fuzz
cargo install cargo-fuzz
```

## Available Fuzz Targets

The project has **19 fuzz targets** across two crates:

### precis-core Targets (6 targets)

Located in `precis-core/fuzz/fuzz_targets/`:

#### **freeform_class_allows** - FreeformClass::allows()
Tests string validation with the permissive FreeformClass.

**What it tests:**
- Character classification (LetterDigits, Symbols, Punctuation, etc.)
- Full string validation
- Edge cases in derived property algorithm

#### **freeform_class_get_value** - FreeformClass::get_value_from_char()
Tests per-character classification.

**What it tests:**
- Individual character property lookup
- All Unicode codepoint categories
- Character boundary cases

#### **freeform_class_codepoint** - FreeformClass::get_value_from_codepoint()
Tests codepoint-based classification with raw u32 values.

**What it tests:**
- Invalid codepoint handling (> U+10FFFF)
- Surrogate pairs (U+D800-U+DFFF)
- Unassigned codepoints
- All valid codepoint ranges

#### **identifier_class_allows** - IdentifierClass::allows()
Tests string validation with the strict IdentifierClass.

**What it tests:**
- Stricter validation rules vs FreeformClass
- Identifier-specific character restrictions
- Full string validation

#### **identifier_class_get_value** - IdentifierClass::get_value_from_char()
Tests per-character classification for identifiers.

**What it tests:**
- Character classification differences vs FreeformClass
- Identifier-specific rules
- Character boundary cases

#### **identifier_class_codepoint** - IdentifierClass::get_value_from_codepoint()
Tests codepoint-based classification for identifiers.

**What it tests:**
- Invalid codepoint handling
- Identifier-specific codepoint restrictions
- All codepoint ranges

### precis-profiles Targets (13 targets)

Located in `precis-profiles/fuzz/fuzz_targets/`:

#### Nickname Profile (4 targets)

**nickname_enforce** - Nickname::enforce()
- Space trimming with various Unicode spaces
- Width mapping edge cases
- BiDi rule validation
- Unicode normalization

**nickname_prepare** - Nickname::prepare()
- Preparation without enforcement
- Normalization edge cases
- Width mapping

**nickname_compare** - Nickname::compare()
- Case-insensitive comparison
- Normalization equivalence
- Comparison of edge cases

**nickname_arbitrary** - Nickname with invalid UTF-8
- Invalid UTF-8 handling
- Multibyte character boundaries
- Lossy conversion edge cases

#### OpaqueString Profile (3 targets)

**opaque_string_enforce** - OpaqueString::enforce()
- Password normalization
- Unicode in passwords
- Special character handling

**opaque_string_prepare** - OpaqueString::prepare()
- Password preparation
- Normalization rules
- Unicode handling

**opaque_string_compare** - OpaqueString::compare()
- Password comparison
- Case-sensitive matching
- Normalization equivalence

#### UsernameCaseMapped Profile (3 targets)

**username_casemapped** - UsernameCaseMapped::enforce()
- Case mapping edge cases
- International usernames
- Username validation rules

**username_casemapped_prepare** - UsernameCaseMapped::prepare()
- Case folding
- Normalization
- Width mapping

**username_casemapped_compare** - UsernameCaseMapped::compare()
- Case-insensitive username comparison
- Normalization equivalence
- International character handling

#### UsernameCasePreserved Profile (3 targets)

**username_casepreserved** - UsernameCasePreserved::enforce()
- Case-sensitive username validation
- International usernames
- Validation rules

**username_casepreserved_prepare** - UsernameCasePreserved::prepare()
- Preparation without case folding
- Normalization
- Width mapping

**username_casepreserved_compare** - UsernameCasePreserved::compare()
- Case-sensitive comparison
- Exact matching rules
- Normalization equivalence

## Running Fuzz Tests

### Quick Start - Run a specific target

```bash
# Go to precis-profiles directory
cd precis-profiles

# Run fuzzing for 60 seconds (basic test)
cargo +nightly fuzz run nickname_enforce -- -max_total_time=60

# Run with specific number of runs
cargo +nightly fuzz run nickname_enforce -- -runs=10000
```

### List available targets

**precis-profiles targets:**
```bash
cd precis-profiles
cargo +nightly fuzz list
```

**Output (13 targets):**
```
nickname_arbitrary
nickname_compare
nickname_enforce
nickname_prepare
opaque_string_compare
opaque_string_enforce
opaque_string_prepare
username_casemapped
username_casemapped_compare
username_casemapped_prepare
username_casepreserved
username_casepreserved_compare
username_casepreserved_prepare
```

**precis-core targets:**
```bash
cd precis-core
cargo +nightly fuzz list
```

**Output (6 targets):**
```
freeform_class_allows
freeform_class_codepoint
freeform_class_get_value
identifier_class_allows
identifier_class_codepoint
identifier_class_get_value
```

### Run all targets (sequentially)

**Fuzz all precis-profiles targets:**
```bash
cd precis-profiles

# Run each target for 60 seconds
for target in $(cargo +nightly fuzz list); do
    echo "Fuzzing $target..."
    cargo +nightly fuzz run $target -- -max_total_time=60
done
```

**Fuzz all precis-core targets:**
```bash
cd precis-core

# Run each target for 60 seconds
for target in $(cargo +nightly fuzz list); do
    echo "Fuzzing $target..."
    cargo +nightly fuzz run $target -- -max_total_time=60
done
```

**Fuzz ALL targets from workspace root:**
```bash
# From project root
cd precis-profiles && for target in $(cargo +nightly fuzz list); do
    echo "Fuzzing profiles/$target..."
    cargo +nightly fuzz run $target -- -max_total_time=60
done

cd ../precis-core && for target in $(cargo +nightly fuzz list); do
    echo "Fuzzing core/$target..."
    cargo +nightly fuzz run $target -- -max_total_time=60
done
```

### Recommended fuzzing durations

**Quick smoke test:**
```bash
# 1 minute per target (~5 minutes total)
cargo +nightly fuzz run nickname_enforce -- -max_total_time=60
```

**Moderate testing:**
```bash
# 5 minutes per target (~25 minutes total)
cargo +nightly fuzz run nickname_enforce -- -max_total_time=300
```

**Extensive testing:**
```bash
# 1 hour per target (run overnight or in CI)
cargo +nightly fuzz run nickname_enforce -- -max_total_time=3600
```

**Continuous fuzzing:**
```bash
# Run indefinitely (Ctrl+C to stop)
cargo +nightly fuzz run nickname_enforce
```

## Understanding Output

### Normal execution (no bugs found)

```
#1      INITED cov: 245 ft: 312 corp: 1/1b exec/s: 0 rss: 32Mb
#8192   pulse  cov: 421 ft: 1823 corp: 45/156b lim: 21 exec/s: 4096 rss: 45Mb
#16384  pulse  cov: 425 ft: 1891 corp: 52/201b lim: 29 exec/s: 5461 rss: 48Mb
```

**Metrics:**
- `cov`: Code coverage (edges covered)
- `ft`: Features (code paths)
- `corp`: Corpus size (interesting inputs saved)
- `exec/s`: Executions per second
- `rss`: Memory usage

### Bug found!

```
==12345==ERROR: libFuzzer: deadly signal
    #0 0x10abcd123 in precis_profiles::nickname::enforce
    #1 0x10abcd456 in LLVMFuzzerTestOneInput
```

When a bug is found:
1. **Crash details** are printed
2. **Minimized input** is saved to `fuzz/artifacts/`
3. **Stack trace** shows where the panic occurred

## Working with Crashes

### View crash artifacts

```bash
ls -la precis-profiles/fuzz/artifacts/nickname_enforce/
```

Crash files are named with the crash type and hash:
```
crash-abc123def456.txt
```

### Reproduce a crash

```bash
# Run fuzzer with specific crashing input
cargo +nightly fuzz run nickname_enforce \
    fuzz/artifacts/nickname_enforce/crash-abc123def456
```

### Debug a crash

```bash
# Build without fuzzing for debugging
cd precis-profiles/fuzz
cargo +nightly build --bin nickname_enforce

# Run under debugger
lldb target/debug/nickname_enforce fuzz/artifacts/nickname_enforce/crash-abc123def456
```

### Minimize crash input

Fuzzer automatically minimizes, but you can re-minimize:

```bash
cargo +nightly fuzz tmin nickname_enforce \
    fuzz/artifacts/nickname_enforce/crash-abc123def456
```

## Corpus Management

The fuzzer builds a **corpus** of interesting inputs that discover new code paths.

### Location

```
precis-profiles/fuzz/corpus/
├── nickname_enforce/
├── nickname_compare/
├── nickname_arbitrary/
├── opaque_string_enforce/
└── username_casemapped/
```

### View corpus

```bash
ls -lh precis-profiles/fuzz/corpus/nickname_enforce/
```

### Seed corpus (optional)

You can provide initial inputs to guide fuzzing:

```bash
mkdir -p precis-profiles/fuzz/corpus/nickname_enforce
echo "alice" > precis-profiles/fuzz/corpus/nickname_enforce/alice
echo "josé" > precis-profiles/fuzz/corpus/nickname_enforce/jose
echo "مرحبا" > precis-profiles/fuzz/corpus/nickname_enforce/arabic
```

### Merge corpus from multiple runs

```bash
cargo +nightly fuzz cmin nickname_enforce
```

This removes duplicate/redundant inputs.

## Advanced Options

### Coverage-guided fuzzing

Fuzzing automatically tracks code coverage and prioritizes inputs that reach new code.

### Dictionary

Create a dictionary of "interesting" tokens:

```bash
cat > precis-profiles/fuzz/dict.txt << 'EOF'
"alice"
"josé"
"@"
"\u{200c}"  # ZWNJ
"\u{200d}"  # ZWJ
EOF
```

Use it:
```bash
cargo +nightly fuzz run nickname_enforce -- -dict=dict.txt
```

### Parallel fuzzing

Run multiple fuzzer instances in parallel:

```bash
# Terminal 1
cargo +nightly fuzz run nickname_enforce -- -jobs=4

# Or use different workers
cargo +nightly fuzz run nickname_enforce -- -workers=4
```

### Memory limit

Limit memory to detect excessive allocations:

```bash
cargo +nightly fuzz run nickname_enforce -- -rss_limit_mb=512
```

### Slow input detection

Find inputs that cause performance issues:

```bash
cargo +nightly fuzz run nickname_enforce -- -timeout=1
```

Inputs taking > 1 second will be reported as "timeout".

## Continuous Fuzzing

### Run overnight

```bash
cd precis-profiles
nohup cargo +nightly fuzz run nickname_enforce &
```

Check progress:
```bash
tail -f nohup.out
```

### OSS-Fuzz Integration (Optional)

For continuous fuzzing infrastructure, consider integrating with [OSS-Fuzz](https://google.github.io/oss-fuzz/):
- Free continuous fuzzing for open source projects
- Runs 24/7 on Google infrastructure
- Automatic bug reports

## Fuzzing in CI/CD

### GitHub Actions (Optional)

Add to `.github/workflows/fuzz.yml`:

```yaml
name: Fuzzing

on:
  schedule:
    # Run nightly at 2 AM UTC
    - cron: '0 2 * * *'
  workflow_dispatch:

jobs:
  fuzz:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install nightly
        run: rustup install nightly

      - name: Install cargo-fuzz
        run: cargo install cargo-fuzz

      - name: Run fuzzing (5 minutes per target)
        run: |
          cd precis-profiles
          for target in $(cargo +nightly fuzz list); do
            echo "Fuzzing $target for 5 minutes..."
            cargo +nightly fuzz run $target -- -max_total_time=300 || true
          done

      - name: Upload artifacts if crash found
        if: failure()
        uses: actions/upload-artifact@v4
        with:
          name: fuzz-artifacts
          path: precis-profiles/fuzz/artifacts/
```

## Best Practices

### 1. Start with short runs

```bash
# Quick smoke test first
cargo +nightly fuzz run nickname_enforce -- -max_total_time=60
```

### 2. Run regularly

Fuzzing is most effective when run continuously:
- After code changes
- Overnight on development machine
- In CI/CD pipeline

### 3. Investigate all crashes

Every crash should be:
1. Reproduced
2. Debugged
3. Fixed
4. Added as a unit test

### 4. Monitor coverage

Higher coverage = better fuzzing:

```bash
cargo +nightly fuzz coverage nickname_enforce
```

### 5. Focus on critical functions

Priority targets:
1. `nickname_enforce` - Most commonly used profile
2. `opaque_string_enforce` - Password security critical
3. `nickname_arbitrary` - Tests invalid UTF-8 robustness

## Troubleshooting

### Error: "cargo-fuzz not found"

```bash
cargo install cargo-fuzz
```

### Error: "requires nightly"

```bash
rustup install nightly
```

### Fuzzing is slow

**Normal speeds:**
- Modern laptop: 1,000 - 10,000 exec/s
- Slower for complex functions

**To improve speed:**
- Use `--release` build: `cargo +nightly fuzz run -O nickname_enforce`
- Reduce corpus size: `cargo +nightly fuzz cmin nickname_enforce`

### Out of memory

Reduce memory limit:
```bash
cargo +nightly fuzz run nickname_enforce -- -rss_limit_mb=512
```

### No new coverage

This is normal after initial fuzzing. The fuzzer has explored most code paths. Consider:
- Longer runs
- Different seed corpus
- New fuzz targets

## What to Do if Fuzzing Finds a Bug

1. **Don't panic** - This is why we fuzz! 🎉

2. **Reproduce the crash:**
   ```bash
   cargo +nightly fuzz run nickname_enforce \
       fuzz/artifacts/nickname_enforce/crash-abc123
   ```

3. **Create a minimal unit test:**
   ```rust
   #[test]
   fn test_fuzzer_found_crash() {
       let input = "..."; // Crashing input
       let result = Nickname::enforce(input);
       assert!(result.is_ok()); // Or expect specific error
   }
   ```

4. **Fix the bug**

5. **Verify fix:**
   ```bash
   # Re-run with same input
   cargo +nightly fuzz run nickname_enforce \
       fuzz/artifacts/nickname_enforce/crash-abc123
   ```

6. **Keep the corpus** - It found a real bug!

## References

- [cargo-fuzz book](https://rust-fuzz.github.io/book/cargo-fuzz.html)
- [libFuzzer documentation](https://llvm.org/docs/LibFuzzer.html)
- [Rust Fuzz Project](https://github.com/rust-fuzz)

---

**Status:** ✅ Fuzzing configured and ready to use
**Targets:** 5 fuzz targets covering main profiles
**Last updated:** 2026-02-08
