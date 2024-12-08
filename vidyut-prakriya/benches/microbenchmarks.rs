use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn last_with_chars(last: &str) -> Option<char> {
    last.chars().rev().next()
}

fn last_with_bytes(last: &str) -> Option<char> {
    last.bytes().rev().next().map(|b| b as char)
}

fn last_with_indexing(last: &str) -> Option<char> {
    let n = last.len();
    if n > 0 {
        Some(last.as_bytes()[n - 1] as char)
    } else {
        None
    }
}

fn string_equality(x: &str) -> bool {
    x == "BU" || x == "asa~" || x == "qukf\\Y" || x == "qulabaZ"
}

fn int_equality(x: usize) -> bool {
    x == 1 || x == 2 || x == 3 || x == 4
}

pub fn get_last_char(c: &mut Criterion) {
    let data = String::from("Bavati");
    c.bench_function("last chars", |b| {
        b.iter(|| last_with_chars(black_box(&data)))
    });
    c.bench_function("last bytes", |b| {
        b.iter(|| last_with_bytes(black_box(&data)))
    });
    c.bench_function("last indexing", |b| {
        b.iter(|| last_with_indexing(black_box(&data)))
    });
}

pub fn test_equality(c: &mut Criterion) {
    let x = 3;
    let y = String::from("qukf\\Y");
    c.bench_function("string equality", |b| {
        b.iter(|| string_equality(black_box(&y)))
    });
    c.bench_function("int equality", |b| b.iter(|| int_equality(black_box(x))));
}

criterion_group!(benches, get_last_char, test_equality);
criterion_main!(benches);
