use crate::{ast::DomElement, html::NonVoidHtmlElement};
use stunseed_derive::HtmlElement;

#[derive(Clone, HtmlElement)]
pub struct Fragment {
    inner: DomElement,
}

#[inline]
pub fn fragment() -> Box<Fragment> {
    Box::new(Fragment {
        inner: DomElement::new_fragment(),
    })
}

unsafe impl NonVoidHtmlElement for Fragment {}
