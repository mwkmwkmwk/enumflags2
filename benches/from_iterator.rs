use criterion::{black_box, criterion_group, criterion_main, Criterion};
use enumflags2::BitFlags;

#[derive(Clone, Copy, BitFlags)]
pub enum Test {
    Flag1 = 1 << 0,
    Flag2 = 1 << 1,
    Flag3 = 1 << 2,
    Flag4 = 1 << 3,
    Flag5 = 1 << 4,
    Flag6 = 1 << 5,
    Flag7 = 1 << 6,
    Flag8 = 1 << 7,
    Flag9 = 1 << 8,
    Flag10 = 1 << 9,
    Flag11 = 1 << 10,
}

pub fn from_iterator(c: &mut Criterion) {
    let v = vec![
        Test::Flag3,
        Test::Flag7,
        Test::Flag5,
        Test::Flag11,
    ];

    let v2 = vec![
        Test::Flag10,
        Test::Flag3,
        Test::Flag1,
        Test::Flag4,
    ];

    c.bench_function("simple iterator", |b| b.iter(|| {
        black_box(&v).iter().copied().collect::<BitFlags<_>>()
    }));

    c.bench_function("chained iterator", |b| b.iter(|| {
        black_box(&v).iter()
            .chain(black_box(&v2).iter())
            .copied()
            .collect::<BitFlags<_>>()
    }));
}

criterion_group!(benches, from_iterator);
criterion_main!(benches);
