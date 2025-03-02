use bears::{Chunks, Dataset, Event, History, Mode, Queue};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn queue_with_event(c: &mut Criterion) {
    let datasets = [
        Dataset::NIUnderlyingDetail,
        Dataset::FixedAssets,
        Dataset::Nipa,
        Dataset::Mne,
    ];
    let queues = datasets
        .iter()
        .map(|d| d.queue().unwrap())
        .collect::<Vec<Queue>>();
    let sizes = queues
        .iter()
        .map(|q| std::mem::size_of_val(q) as u64)
        .collect::<Vec<u64>>();
    let histories = datasets
        .iter()
        .map(|d| History::try_from((*d, Mode::Load)).unwrap())
        .collect::<Vec<History>>();
    let events = histories
        .iter()
        .map(|h| h.values().take(1).collect::<Vec<&Event>>().pop().unwrap())
        .collect::<Vec<&Event>>();

    let mut group = c.benchmark_group("queue sizes");
    for (i, queue) in queues.iter().enumerate() {
        group.throughput(criterion::Throughput::Bytes(sizes[i]));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(format!("{} - {}", datasets[i], "serial")),
            queue,
            |b, queue| {
                b.iter(|| queue.clone().with_event(events[i]));
            },
        );
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(format!("{} - {}", datasets[i], "parallel")),
            queue,
            |b, queue| {
                b.iter(|| queue.clone().with_event_par(events[i]));
            },
        );
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(format!("{} - {}", datasets[i], "reference")),
            queue,
            |b, queue| {
                b.iter(|| queue.clone().with_event_ref(events[i]));
            },
        );
    }
    group.finish();
}

pub fn chunk_with_event(c: &mut Criterion) {
    let datasets = [
        Dataset::NIUnderlyingDetail,
        Dataset::FixedAssets,
        // Dataset::Nipa,
        // Dataset::Mne,
    ];
    let queues = datasets
        .iter()
        .map(|d| d.queue().unwrap())
        .collect::<Vec<Queue>>();
    let sizes = queues
        .iter()
        .map(|q| std::mem::size_of_val(q) as u64)
        .collect::<Vec<u64>>();
    let histories = datasets
        .iter()
        .map(|d| History::try_from((*d, Mode::Load)).unwrap())
        .collect::<Vec<History>>();
    let chunks = histories.iter().map(|h| h.iter()).collect::<Vec<Chunks>>();

    let mut group = c.benchmark_group("queue sizes");
    for (i, chunk) in chunks.iter().enumerate() {
        group.throughput(criterion::Throughput::Bytes(sizes[i]));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(format!(
                "{} - {}",
                datasets[i], "chunks reference"
            )),
            chunk,
            |b, chunk| {
                b.iter(|| chunk.with_queue(&queues[i]));
            },
        );
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(format!(
                "{} - {}",
                datasets[i], "chunks single"
            )),
            chunk,
            |b, chunk| {
                b.iter(|| chunk.with_queue_single(&queues[i]));
            },
        );
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(format!(
                "{} - {}",
                datasets[i], "chunks parallel"
            )),
            chunk,
            |b, chunk| {
                b.iter(|| chunk.with_queue_par(&queues[i]));
            },
        );
    }
    group.finish();
}

// criterion_group!(benches, queue_with_event, chunk_with_event);
criterion_group!(benches, chunk_with_event);
criterion_main!(benches);
