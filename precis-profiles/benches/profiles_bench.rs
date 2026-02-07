use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use precis_core::profile::PrecisFastInvocation;
use precis_profiles::{Nickname, OpaqueString, UsernameCaseMapped, UsernameCasePreserved};

fn bench_nickname(c: &mut Criterion) {
    let test_strings = vec![
        ("alice", "ASCII simple"),
        ("Alice123", "ASCII mixed"),
        ("  alice  ", "ASCII with spaces"),
        ("Алиса", "Cyrillic"),
        ("爱丽丝", "CJK"),
        ("alice@example", "ASCII with symbol"),
    ];

    let mut group = c.benchmark_group("Nickname");

    // Benchmark enforce
    for (s, name) in &test_strings {
        group.bench_with_input(BenchmarkId::new("enforce", name), s, |b, &s| {
            b.iter(|| Nickname::enforce(black_box(s)))
        });
    }

    // Benchmark prepare
    for (s, name) in &test_strings {
        group.bench_with_input(BenchmarkId::new("prepare", name), s, |b, &s| {
            b.iter(|| Nickname::prepare(black_box(s)))
        });
    }

    // Benchmark compare
    group.bench_function("compare", |b| {
        b.iter(|| Nickname::compare(black_box("alice"), black_box("Alice")))
    });

    group.finish();
}

fn bench_username_casemapped(c: &mut Criterion) {
    let test_strings = vec![
        ("alice", "ASCII simple"),
        ("Alice123", "ASCII mixed"),
        ("alice_bob", "ASCII with underscore"),
        ("alice-bob", "ASCII with dash"),
        ("алиса", "Cyrillic lowercase"),
    ];

    let mut group = c.benchmark_group("UsernameCaseMapped");

    for (s, name) in &test_strings {
        group.bench_with_input(BenchmarkId::new("enforce", name), s, |b, &s| {
            b.iter(|| UsernameCaseMapped::enforce(black_box(s)))
        });
    }

    // Benchmark compare (case-insensitive)
    group.bench_function("compare_case_insensitive", |b| {
        b.iter(|| UsernameCaseMapped::compare(black_box("Alice"), black_box("alice")))
    });

    group.finish();
}

fn bench_username_casepreserved(c: &mut Criterion) {
    let test_strings = vec![
        ("alice", "ASCII simple"),
        ("Alice123", "ASCII mixed"),
        ("alice_bob", "ASCII with underscore"),
    ];

    let mut group = c.benchmark_group("UsernameCasePreserved");

    for (s, name) in &test_strings {
        group.bench_with_input(BenchmarkId::new("enforce", name), s, |b, &s| {
            b.iter(|| UsernameCasePreserved::enforce(black_box(s)))
        });
    }

    // Benchmark compare (case-sensitive)
    group.bench_function("compare_case_sensitive", |b| {
        b.iter(|| UsernameCasePreserved::compare(black_box("Alice"), black_box("alice")))
    });

    group.finish();
}

fn bench_opaquestring(c: &mut Criterion) {
    let test_strings = vec![
        ("password123", "ASCII simple"),
        ("P@ssw0rd!", "ASCII with symbols"),
        ("пароль123", "Cyrillic"),
        ("密码123", "CJK"),
        ("correct horse battery staple", "ASCII with spaces"),
    ];

    let mut group = c.benchmark_group("OpaqueString");

    for (s, name) in &test_strings {
        group.bench_with_input(BenchmarkId::new("enforce", name), s, |b, &s| {
            b.iter(|| OpaqueString::enforce(black_box(s)))
        });
    }

    // Benchmark compare
    group.bench_function("compare", |b| {
        b.iter(|| OpaqueString::compare(black_box("password"), black_box("password")))
    });

    group.finish();
}

fn bench_enforce_length(c: &mut Criterion) {
    let strings = vec![
        ("a".repeat(10), "10 chars"),
        ("a".repeat(50), "50 chars"),
        ("a".repeat(100), "100 chars"),
        ("a".repeat(500), "500 chars"),
    ];

    let mut group = c.benchmark_group("enforce_by_length");

    for (s, name) in &strings {
        group.bench_with_input(BenchmarkId::new("Nickname", name), s, |b, s| {
            b.iter(|| Nickname::enforce(black_box(s.as_str())))
        });

        group.bench_with_input(BenchmarkId::new("UsernameCaseMapped", name), s, |b, s| {
            b.iter(|| UsernameCaseMapped::enforce(black_box(s.as_str())))
        });

        group.bench_with_input(BenchmarkId::new("OpaqueString", name), s, |b, s| {
            b.iter(|| OpaqueString::enforce(black_box(s.as_str())))
        });
    }

    group.finish();
}

fn bench_unicode_complexity(c: &mut Criterion) {
    let test_strings = vec![
        ("hello", "ASCII"),
        ("Здравствуй", "Cyrillic"),
        ("你好世界", "CJK"),
        ("مرحبا", "Arabic"),
        ("שלום", "Hebrew"),
        ("こんにちは", "Hiragana"),
        ("안녕하세요", "Hangul"),
        ("👋🌍", "Emoji"),
    ];

    let mut group = c.benchmark_group("unicode_complexity");

    for (s, name) in &test_strings {
        group.bench_with_input(BenchmarkId::from_parameter(name), s, |b, &s| {
            b.iter(|| Nickname::enforce(black_box(s)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_nickname,
    bench_username_casemapped,
    bench_username_casepreserved,
    bench_opaquestring,
    bench_enforce_length,
    bench_unicode_complexity
);
criterion_main!(benches);
