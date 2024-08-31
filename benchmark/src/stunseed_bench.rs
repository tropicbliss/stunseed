use criterion::{measurement::WallTime, BenchmarkGroup};
use std::hint::black_box;
use stunseed::prelude::*;

struct Teams {
    year: u16,
    teams: Vec<Team>,
}

struct Team {
    name: String,
    score: u8,
}

fn teams_render(teams: &Teams) -> String {
    html()
        .children(vec![
            head().title(teams.year.to_string()),
            body().children(vec![
                h1().child(text(format!("CSL {}", teams.year))),
                ul().children(
                    teams
                        .teams
                        .iter()
                        .enumerate()
                        .map(|(idx, team)| {
                            let mut res = li();
                            if idx == 0 {
                                res = res.class("champion");
                            }
                            res.children(vec![
                                b().child(text(team.name.clone())),
                                text(format!(": {}", team.score)),
                            ])
                        })
                        .collect(),
                ),
            ]),
        ])
        .build()
}

fn big_table_render(tablez: &Vec<Vec<usize>>) -> String {
    html()
        .child(
            table().children(
                tablez
                    .iter()
                    .map(|row: &Vec<usize>| {
                        tr().children(
                            row.iter()
                                .map(|col| td().child(text(col.to_string())))
                                .collect(),
                        )
                    })
                    .collect(),
            ),
        )
        .build()
}

fn gen_big_table(size: usize) -> Vec<Vec<usize>> {
    let mut table = Vec::with_capacity(size);
    for _ in 0..size {
        let mut inner = Vec::with_capacity(size);
        for i in 0..size {
            inner.push(i);
        }
        table.push(inner);
    }
    table
}

pub fn bench_teams(group: &mut BenchmarkGroup<'_, WallTime>) {
    let teams = Teams {
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
    group.bench_function("stunseed", |b| b.iter(|| teams_render(black_box(&teams))));
}

pub fn bench_big_table(group: &mut BenchmarkGroup<'_, WallTime>) {
    let big_table = gen_big_table(100);
    group.bench_function("stunseed", |b| {
        b.iter(|| big_table_render(black_box(&big_table)))
    });
}
