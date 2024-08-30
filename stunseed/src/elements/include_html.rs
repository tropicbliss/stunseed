#[macro_export]
macro_rules! include_html {
    ($path:expr) => {{
        let source = include_str!($path);
        embed_html!(source)
    }};
}

#[macro_export]
macro_rules! embed_html {
    ($source:expr) => {{
        crate::elements::text::text($source)
    }};
}
