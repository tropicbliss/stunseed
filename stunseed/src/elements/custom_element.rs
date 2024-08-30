use crate::{ast::DomElement, html::NonVoidHtmlElement, prelude::ContainsGlobalAttributes};
use stunseed_derive::HtmlElement;

#[derive(Clone, HtmlElement)]
pub struct CustomElement {
    inner: DomElement,
}

#[inline]
pub fn custom_element(name: &'static str) -> Box<CustomElement> {
    Box::new(CustomElement {
        inner: DomElement::new(name),
    })
}

unsafe impl NonVoidHtmlElement for CustomElement {}
impl ContainsGlobalAttributes for CustomElement {}
