fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

//TODO Code to benchmark
// use wordseg::Segmenter;

// pub fn old_fit(&'a mut self, segmented: &[String]) -> &'a mut Self {
    // for word in segmented.iter() {
        // if word.graphemes(true).count() > 1 {
            // self.model.insert(word.to_owned());
        // }
    // }

    // self
// }
