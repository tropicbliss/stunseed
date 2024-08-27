# stunseed

A WIP ergonomic HTML builder.

```rs
fn generate_html() -> String {
    html().children(vec![component()]).build()
}

fn component() -> Box<dyn HtmlNode> {
    div().children(vec![a()
        .href("https://www.example.com")
        .target(AnchorTarget::Blank)])
}
```

## Credits
- [typed-html crate](https://github.com/bodil/typed-html)
- [html crate](https://github.com/yoshuawuyts/html)
