use crate::{
    ast::{AttributeValue, DomElement, DomNode},
    attributes::*,
    html::{HtmlNode, NonVoidHtmlElement},
    utils,
};
use ammonia::clean;
use smartstring::alias::String as SmartString;
use std::sync::OnceLock;

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

#[macro_export]
macro_rules! declare_element {
    ($name:ident, $doc:expr) => {
        ::paste::paste! {
            #[derive(Clone)]
            pub struct [<$name:camel>] {
                inner: $crate::ast::DomElement
            }

            impl_html_element!([<$name:camel>]);

            #[doc = $doc]
            pub fn [<$name:lower>]() -> Box<[<$name:camel>]> {
                Box::new([<$name:camel>] {
                    inner: $crate::ast::DomElement::new(stringify!([<$name:lower>])),
                })
            }

            unsafe impl $crate::html::NonVoidHtmlElement for [<$name:camel>] {}
        }
    };

    ($name:ident, $doc:expr, $( $attr:ty ),*) => {
        declare_element!($name, $doc);
        ::paste::paste! {
            $(
                impl $attr for [<$name:camel>] {}
            )*
        }
    };
}

#[macro_export]
macro_rules! declare_element_void {
    ($name:ident, $doc:expr) => {
        ::paste::paste! {
            #[derive(Clone)]
            pub struct [<$name:camel>] {
                inner: $crate::ast::DomElement
            }

            impl_html_element!([<$name:camel>]);

            #[doc = $doc]
            pub fn [<$name:lower>]() -> Box<[<$name:camel>]> {
                Box::new([<$name:camel>] {
                    inner: $crate::ast::DomElement::new_void(stringify!([<$name:lower>])),
                })
            }
        }
    };

    ($name:ident, $doc:expr, $( $attr:ty ),*) => {
        declare_element_void!($name, $doc);
        ::paste::paste! {
            $(
                impl $attr for [<$name:camel>] {}
            )*
        }
    }
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
pub struct Text(SmartString);

#[inline]
pub fn text<T>(value: T) -> Box<Text>
where
    T: Into<SmartString>,
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

// HTML
#[derive(Clone)]
pub struct Html {
    inner: DomElement,
}

impl Html {
    #[inline]
    pub fn role<T>(mut self, value: T) -> Self
    where
        T: Into<SmartString>,
    {
        self.inner
            .insert_attribute("role", AttributeValue::KeyValuePair(value.into()));
        self
    }

    #[inline]
    pub fn lang<T>(mut self, value: T) -> Self
    where
        T: Into<SmartString>,
    {
        self.inner
            .insert_attribute("lang", AttributeValue::KeyValuePair(value.into()));
        self
    }

    #[inline]
    pub fn children(mut self, children: Vec<Box<dyn HtmlNode>>) -> Self
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

    pub fn build(self) -> String {
        clean(&self.build_unsanitized())
    }

    pub fn build_unsanitized(self) -> String {
        utils::html_builder(self.inner)
    }
}

#[inline]
pub fn html() -> Html {
    Html {
        inner: DomElement::new("html"),
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
