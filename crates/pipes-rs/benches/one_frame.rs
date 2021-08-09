use config::Config;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use pipes_rs::App;
use std::time::Duration;
use terminal::VoidBackend;

const MAX_NUM_PIPES: u32 = 500;
const NUM_BENCHES: u32 = 10;

fn one_frame(c: &mut Criterion) {
    let mut group = c.benchmark_group("one_frame");

    group
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_millis(1000));

    for n in (1..=NUM_BENCHES).map(|n| n * (MAX_NUM_PIPES / NUM_BENCHES)) {
        let config = Config {
            delay_ms: Some(0),
            num_pipes: Some(n),
            ..Default::default()
        };
        let mut app = App::new(VoidBackend, config).unwrap();
        let mut pipes = app.create_pipes();

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &_| {
            b.iter(|| {
                let _ = app.tick_loop(&mut pipes).unwrap();
            })
        });
    }

    group.finish();
}

criterion_group!(benches, one_frame);
criterion_main!(benches);
