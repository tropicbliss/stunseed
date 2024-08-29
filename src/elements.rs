use crate::{
    ast::{DomElement, DomNode},
    html::{HtmlNode, NonVoidHtmlElement},
    utils,
};
use ammonia::clean;
use std::{borrow::Cow, sync::OnceLock};

#[macro_export]
macro_rules! impl_html_element {
    ($name:ty) => {
        impl $crate::html::HtmlElement for $name {
            fn get_dom_element_mut(&mut self) -> &mut $crate::ast::DomElement {
                &mut self.inner
            }

            fn get_dom_element(&self) -> $crate::ast::DomElement {
                self.inner.clone()
            }
        }
    };
}

pub fn builder<F>(closure: F) -> Box<dyn HtmlNode>
where
    F: FnOnce() -> Box<dyn HtmlNode>,
{
    closure()
}

pub fn constify(closure: fn() -> Box<dyn HtmlNode>) -> Box<dyn HtmlNode> {
    static CONSTANT: OnceLock<Box<dyn HtmlNode>> = OnceLock::new();
    dyn_clone::clone_box(&**CONSTANT.get_or_init(closure))
}

// Text
#[derive(Clone)]
pub struct Text(Cow<'static, str>);

#[inline]
pub fn text<T>(value: T) -> Box<Text>
where
    T: Into<Cow<'static, str>>,
{
    Box::new(Text(value.into()))
}

#[macro_export]
macro_rules! include_html {
    ($path:expr) => {{
        let source = include_str!($path);
        text(source)
    }};
}

impl HtmlNode for Text {
    fn get_dom_node(&self) -> DomNode {
        DomNode::TextNode(self.0.clone())
    }
}

// CustomElement
#[derive(Clone)]
pub struct CustomElement {
    inner: DomElement,
}

impl_html_element!(CustomElement);

#[inline]
pub fn custom_element(name: &'static str) -> Box<CustomElement> {
    Box::new(CustomElement {
        inner: DomElement::new(name),
    })
}

unsafe impl NonVoidHtmlElement for CustomElement {}

// HTML elements
include!("generated/gen_elements.rs");

impl Html {
    pub fn build(self) -> String {
        clean(&self.build_unsanitized())
    }

    pub fn build_unsanitized(self) -> String {
        utils::html_builder(self.inner)
    }

    #[inline]
    pub fn child(mut self: Box<Self>, child: Box<dyn HtmlNode>) -> Box<Self>
    where
        Self: 'static + Send + Sync + Clone,
    {
        let dom_node = child.get_dom_node();
        let children = utils::create_single_element_vec(dom_node);
        unsafe {
            self.inner.set_children(children);
        }
        self
    }

    #[inline]
    pub fn children(mut self: Box<Self>, children: Vec<Box<dyn HtmlNode>>) -> Box<Self>
    where
        Self: 'static + Send + Sync + Clone,
    {
        let dom_nodes = children
            .into_iter()
            .map(|node| node.get_dom_node())
            .collect();
        unsafe {
            self.inner.set_children(dom_nodes);
        }
        self
    }
}
