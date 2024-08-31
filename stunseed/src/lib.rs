extern crate self as stunseed;

pub mod ast;
pub mod attributes;
pub mod elements;
pub mod html;
mod utils;

pub use stunseed_derive::HtmlElement;

pub mod prelude {
    pub use crate::attributes::*;
    pub use crate::elements::*;
    pub use crate::html::*;
}

#[cfg(test)]
mod tests {
    use stunseed::prelude::*;

    struct Teams {
        year: u16,
        teams: Vec<Team>,
    }

    struct Team {
        name: String,
        score: u8,
    }

    #[test]
    fn teams() {
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
        let html = html()
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
            .build();
        assert_eq!(html, "<!DOCTYPE html><html><head title=\"2015\"></head><body><h1>CSL 2015</h1><ul><li class=\"champion\"><b>Jiangsu</b>: 43</li><li><b>Beijing</b>: 27</li><li><b>Guangzhou</b>: 22</li><li><b>Shandong</b>: 12</li></ul></body></html>");
    }

    #[test]
    fn big_table() {
        let size = 10;
        let mut big_table = Vec::with_capacity(size);
        for _ in 0..size {
            let mut inner = Vec::with_capacity(size);
            for i in 0..size {
                inner.push(i);
            }
            big_table.push(inner);
        }
        let html = html()
            .child(
                table().children(
                    big_table
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
            .build();
        assert_eq!(html, "<!DOCTYPE html><html><table><tr><td>0</td><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td><td>6</td><td>7</td><td>8</td><td>9</td></tr><tr><td>0</td><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td><td>6</td><td>7</td><td>8</td><td>9</td></tr><tr><td>0</td><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td><td>6</td><td>7</td><td>8</td><td>9</td></tr><tr><td>0</td><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td><td>6</td><td>7</td><td>8</td><td>9</td></tr><tr><td>0</td><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td><td>6</td><td>7</td><td>8</td><td>9</td></tr><tr><td>0</td><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td><td>6</td><td>7</td><td>8</td><td>9</td></tr><tr><td>0</td><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td><td>6</td><td>7</td><td>8</td><td>9</td></tr><tr><td>0</td><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td><td>6</td><td>7</td><td>8</td><td>9</td></tr><tr><td>0</td><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td><td>6</td><td>7</td><td>8</td><td>9</td></tr><tr><td>0</td><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td><td>6</td><td>7</td><td>8</td><td>9</td></tr></table></html>");
    }
}
