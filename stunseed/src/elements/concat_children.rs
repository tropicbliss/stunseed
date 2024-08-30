#[macro_export]
macro_rules! concat_children {
    ( $( $vec:expr ),* ) => {{
        let mut result: Vec<Box<dyn crate::html::HtmlNode>> = Vec::new();
        $(
            result.extend($vec);
        )*
        result
    }};
}
