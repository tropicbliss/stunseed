use askama::Template;
use criterion::{measurement::WallTime, BenchmarkGroup};
use std::hint::black_box;

fn gen_big_table(size: usize) -> BigTable {
    let mut table = Vec::with_capacity(size);
    for _ in 0..size {
        let mut inner = Vec::with_capacity(size);
        for i in 0..size {
            inner.push(i);
        }
        table.push(inner);
    }
    BigTable { table }
}

#[derive(Template)]
#[template(path = "big-table.html")]
struct BigTable {
    table: Vec<Vec<usize>>,
}

#[derive(Template)]
#[template(path = "teams.html")]
struct Teams {
    year: u16,
    teams: Vec<Team>,
}

struct Team {
    name: String,
    score: u8,
}

pub fn bench_teams(group: &mut BenchmarkGroup<'_, WallTime>) {
    let ctx = Teams {
        year: 2015,
        teams: vec![
            Team {
                name: "Jiangsu".into(),
                score: 43,
            },
            Team {
                name: "Beijing".into(),
                score: 27,
            },
            Team {
                name: "Guangzhou".into(),
                score: 22,
            },
            Team {
                name: "Shandong".into(),
                score: 12,
            },
        ],
    };
    group.bench_function("askama", |b| {
        b.iter(|| black_box(&ctx).render().unwrap());
    });
}

pub fn bench_big_table(group: &mut BenchmarkGroup<'_, WallTime>) {
    let ctx = gen_big_table(100);
    group.bench_function("askama", |b| {
        b.iter(|| black_box(&ctx).render().unwrap());
    });
}
