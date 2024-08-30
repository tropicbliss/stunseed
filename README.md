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
- [html crate](https://github.com/yoshuawuyts/html)
