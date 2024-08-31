use benchmark::{askama_bench, stunseed_bench};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    {
        let mut teams_group = c.benchmark_group("teams");
        askama_bench::bench_teams(&mut teams_group);
        stunseed_bench::bench_teams(&mut teams_group);
    }
    {
        let mut big_table_group = c.benchmark_group("big_table");
        askama_bench::bench_big_table(&mut big_table_group);
        stunseed_bench::bench_big_table(&mut big_table_group);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
