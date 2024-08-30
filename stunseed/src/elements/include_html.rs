#[macro_export]
macro_rules! include_html {
    ($path:expr) => {{
        let source = include_str!($path);
        crate::elements::text(source)
    }};
}
