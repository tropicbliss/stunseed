use crate::{ast::DomNode, html::HtmlNode};
use std::borrow::Cow;

#[derive(Clone)]
pub struct EmbeddedHtml(Cow<'static, str>);

#[inline]
pub fn embed_html<T>(value: T) -> Box<EmbeddedHtml>
where
    T: Into<Cow<'static, str>>,
{
    Box::new(EmbeddedHtml(value.into()))
}

impl HtmlNode for EmbeddedHtml {
    fn get_dom_node(&self) -> DomNode {
        DomNode::Html(self.0.clone())
    }
}

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
        stunseed::elements::include_html::embed_html($source)
    }};
}
