use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::{
    arch::x86_64::{_mm_add_ps, _mm_loadu_ps, _mm_store_ps},
    f32::consts::PI,
};

const ARRAY_SIZE: usize = 1_000_000;

fn sum_arrays_simd(a: &Vec<f32>, b: &Vec<f32>, result: &mut Vec<f32>) {
    assert_eq!(a.len(), b.len());
    assert_eq!(b.len(), result.len());

    // align array for SIMD
    let simd_chunks = a.len() / 4 * 4;

    for i in (0..simd_chunks).step_by(4) {
        unsafe {
            let a_chunk = _mm_loadu_ps(&a[i]);
            let b_chunk = _mm_loadu_ps(&b[i]);

            let sum_chunk = _mm_add_ps(a_chunk, b_chunk);

            _mm_store_ps(&mut result[i], sum_chunk);
        }

        for i in simd_chunks..a.len() {
            result[i] = a[i] + b[i]
        }
    }
}

fn sum_arrays_scalar(a: &Vec<f32>, b: &Vec<f32>, result: &mut Vec<f32>) {
    for i in 0..a.len() {
        result[i] = a[i] + b[i]
    }
}

fn benchmark_simd(c: &mut Criterion) {
    let vec1 = (0..ARRAY_SIZE).map(|n| n as f32).collect();
    let vec2 = (0..ARRAY_SIZE).map(|n| n as f32 * PI).collect();

    let mut result = vec![0.0; ARRAY_SIZE];

    c.bench_function("sum_arrays_simd", |b| {
        b.iter(|| sum_arrays_simd(black_box(&vec1), black_box(&vec2), black_box(&mut result)))
    });
}

fn benchmark_scalar(c: &mut Criterion) {
    let vec1 = (0..ARRAY_SIZE).map(|n| n as f32).collect();
    let vec2 = (0..ARRAY_SIZE).map(|n| n as f32 * PI).collect();

    let mut result = vec![0.0; ARRAY_SIZE];

    c.bench_function("sum_arrays_scalar", |b| {
        b.iter(|| sum_arrays_scalar(black_box(&vec1), black_box(&vec2), black_box(&mut result)))
    });
}

criterion_group!(benches, benchmark_simd, benchmark_scalar);
criterion_main!(benches);
