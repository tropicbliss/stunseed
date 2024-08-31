use criterion::{criterion_group, criterion_main, Criterion};
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

fn criterion_benchmark(c: &mut Criterion) {
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
    c.bench_function("html_gen", |b| b.iter(|| teams_render(black_box(&teams))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
