use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use precis_core::{FreeformClass, IdentifierClass, StringClass};

fn bench_get_value_from_char(c: &mut Criterion) {
    let id_class = IdentifierClass::default();
    let ff_class = FreeformClass::default();

    let test_chars = vec![
        ('a', "ASCII lowercase"),
        ('A', "ASCII uppercase"),
        ('5', "ASCII digit"),
        ('α', "Greek letter"),
        ('中', "CJK ideograph"),
        (' ', "Space"),
        ('\u{200C}', "ZWNJ"),
    ];

    let mut group = c.benchmark_group("get_value_from_char");

    for (ch, name) in test_chars {
        group.bench_with_input(BenchmarkId::new("IdentifierClass", name), &ch, |b, &ch| {
            b.iter(|| id_class.get_value_from_char(black_box(ch)))
        });

        group.bench_with_input(BenchmarkId::new("FreeformClass", name), &ch, |b, &ch| {
            b.iter(|| ff_class.get_value_from_char(black_box(ch)))
        });
    }

    group.finish();
}

fn bench_get_value_from_codepoint(c: &mut Criterion) {
    let id_class = IdentifierClass::default();

    let test_codepoints = vec![
        (0x0061, "ASCII lowercase"),
        (0x0041, "ASCII uppercase"),
        (0x03B1, "Greek alpha"),
        (0x4E2D, "CJK ideograph"),
        (0x0020, "Space"),
        (0x200C, "ZWNJ"),
    ];

    let mut group = c.benchmark_group("get_value_from_codepoint");

    for (cp, name) in test_codepoints {
        group.bench_with_input(BenchmarkId::from_parameter(name), &cp, |b, &cp| {
            b.iter(|| id_class.get_value_from_codepoint(black_box(cp)))
        });
    }

    group.finish();
}

fn bench_allows(c: &mut Criterion) {
    let id_class = IdentifierClass::default();
    let ff_class = FreeformClass::default();

    let test_strings = vec![
        ("hello", "ASCII simple"),
        ("hello123", "ASCII alphanumeric"),
        ("Здравствуй", "Cyrillic"),
        ("你好世界", "CJK"),
        ("hello world", "ASCII with space"),
        ("user@example.com", "Email-like"),
    ];

    let mut group = c.benchmark_group("allows");

    for (s, name) in test_strings {
        group.bench_with_input(BenchmarkId::new("IdentifierClass", name), &s, |b, &s| {
            b.iter(|| id_class.allows(black_box(s)))
        });

        group.bench_with_input(BenchmarkId::new("FreeformClass", name), &s, |b, &s| {
            b.iter(|| ff_class.allows(black_box(s)))
        });
    }

    group.finish();
}

fn bench_allows_length(c: &mut Criterion) {
    let id_class = IdentifierClass::default();

    let strings = vec![
        ("a".repeat(10), "10 chars"),
        ("a".repeat(50), "50 chars"),
        ("a".repeat(100), "100 chars"),
        ("a".repeat(500), "500 chars"),
    ];

    let mut group = c.benchmark_group("allows_by_length");

    for (s, name) in &strings {
        group.bench_with_input(BenchmarkId::from_parameter(name), s, |b, s| {
            b.iter(|| id_class.allows(black_box(s.as_str())))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_get_value_from_char,
    bench_get_value_from_codepoint,
    bench_allows,
    bench_allows_length
);
criterion_main!(benches);
