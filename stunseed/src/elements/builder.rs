use crate::html::HtmlNode;

#[inline]
pub fn builder<F>(closure: F) -> Box<dyn HtmlNode>
where
    F: FnOnce() -> Box<dyn HtmlNode>,
{
    closure()
}
