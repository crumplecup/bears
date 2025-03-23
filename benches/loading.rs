use bears::{Dataset, History, Mode};

pub fn loading(c: &mut criterion::Criterion) {
    let dataset = Dataset::Ita;
    let downloads = History::try_from((dataset, Mode::Download)).unwrap();
    let mut group = c.benchmark_group("loading");
    group.bench_with_input(
        criterion::BenchmarkId::from_parameter(format!("{} - {}", dataset, "async")),
        &dataset,
        |b, dataset| {
            b.iter(|| dataset.initial_load(Some(&downloads)));
        },
    );
    group.bench_with_input(
        criterion::BenchmarkId::from_parameter(format!("{} - {}", dataset, "parallel")),
        &dataset,
        |b, dataset| {
            b.iter(|| dataset.initial_load_par(Some(&downloads)));
        },
    );
}

criterion::criterion_group!(benches, loading);
criterion::criterion_main!(benches);
