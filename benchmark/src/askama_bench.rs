use askama::Template;
use criterion::{measurement::WallTime, BenchmarkGroup};

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

fn big_table(b: &mut criterion::Bencher<'_>, ctx: &BigTable) {
    b.iter(|| ctx.render().unwrap());
}

#[derive(Template)]
#[template(path = "big-table.html")]
struct BigTable {
    table: Vec<Vec<usize>>,
}

fn teams(b: &mut criterion::Bencher<'_>, teams: &Teams) {
    b.iter(|| teams.render().unwrap());
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
    let team_input = Teams {
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
    group.bench_with_input("askama", &team_input, teams);
}

pub fn bench_big_table(group: &mut BenchmarkGroup<'_, WallTime>) {
    let big_table_input = gen_big_table(100);
    group.bench_with_input("askama", &big_table_input, big_table);
}
