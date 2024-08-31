# stunseed

A WIP ergonomic HTML builder.

```rs
fn generate_html() -> String {
    html().children(vec![component()]).build()
}

fn component() -> Box<dyn HtmlNode> {
    div().children(vec![a()
        .href("https://www.example.com")
        .target("_blank")])
}
```

## Credits
- [askama](https://github.com/djc/askama)
- [html crate](https://github.com/yoshuawuyts/html)
- [template-benchmarks-rs](https://github.com/rosetta-rs/template-benchmarks-rs)
