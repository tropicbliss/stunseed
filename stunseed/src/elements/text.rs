use crate::{ast::DomNode, html::HtmlNode};
use std::borrow::Cow;

#[derive(Clone)]
pub struct Text(Cow<'static, str>);

#[inline]
pub fn text<T>(value: T) -> Box<Text>
where
    T: Into<Cow<'static, str>>,
{
    Box::new(Text(value.into()))
}

impl HtmlNode for Text {
    fn get_dom_node(&self) -> DomNode {
        DomNode::TextNode(self.0.clone())
    }
}
