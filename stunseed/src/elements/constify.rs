use crate::html::HtmlNode;
use std::sync::OnceLock;

#[inline]
pub fn constify(closure: fn() -> Box<dyn HtmlNode>) -> Box<dyn HtmlNode> {
    static CONSTANT: OnceLock<Box<dyn HtmlNode>> = OnceLock::new();
    dyn_clone::clone_box(&**CONSTANT.get_or_init(closure))
}

#[macro_export]
macro_rules! constify {
    ( $x:expr ) => {
        stunseed::elements::constify::constify(|| $x)
    };
}
