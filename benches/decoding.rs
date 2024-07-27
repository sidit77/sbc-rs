use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sbc_rs::Decoder;

fn decode(data: &[u8]) {
    let mut decoder = Decoder::new(data.to_vec());
    while let Some(frame) = decoder.next_frame() {
        black_box(frame);
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let data = std::fs::read("testcases/sbc_test_01.sbc").unwrap();
    c.bench_function("decode", |b| b.iter(|| decode(&data)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);