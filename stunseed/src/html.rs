use std::borrow::Cow;

use crate::{
    ast::{AttributeValue, DomElement, DomNode},
    utils,
};
use dyn_clone::DynClone;

pub trait HtmlElement: Clone {
    fn get_dom_element_mut(&mut self) -> &mut DomElement;

    fn get_dom_element(&self) -> DomElement;

    #[inline]
    fn add_custom_attribute<T>(mut self: Box<Self>, key: &'static str, value: T) -> Box<Self>
    where
        T: Into<Cow<'static, str>>,
    {
        self.get_dom_element_mut()
            .insert_attribute(key, AttributeValue::KeyValuePair(value.into()));
        self
    }

    #[inline]
    fn add_custom_integer_attribute(self: Box<Self>, key: &'static str, value: i64) -> Box<Self> {
        self.add_custom_attribute(key, value.to_string())
    }

    #[inline]
    fn add_custom_float_attribute(self: Box<Self>, key: &'static str, value: f64) -> Box<Self> {
        self.add_custom_attribute(key, value.to_string())
    }

    #[inline]
    fn add_custom_bool_attribute(mut self: Box<Self>, key: &'static str, value: bool) -> Box<Self> {
        self.get_dom_element_mut()
            .insert_attribute(key, AttributeValue::BooleanAttribute(value));
        self
    }
}

#[macro_export]
macro_rules! data_key {
    ($key:expr) => {
        concat!("data-", $key)
    };
}

impl<T> HtmlNode for T
where
    T: HtmlElement + Send + Sync + DynClone,
{
    fn get_dom_node(&self) -> DomNode {
        DomNode::Element(self.get_dom_element())
    }
}

pub trait HtmlNode: Send + Sync + DynClone {
    fn get_dom_node(&self) -> DomNode;

    fn upcast(self: Box<Self>) -> Box<dyn HtmlNode>
    where
        Self: Sized + 'static,
    {
        self as Box<dyn HtmlNode>
    }
}

pub unsafe trait NonVoidHtmlElement: HtmlElement {
    #[inline]
    fn child(mut self, child: Box<dyn HtmlNode>) -> Box<dyn HtmlNode>
    where
        Self: 'static + Send + Sync + Clone,
    {
        let dom_node = child.get_dom_node();
        let children = utils::create_single_element_vec(dom_node);
        unsafe {
            self.get_dom_element_mut().set_children(children);
        }
        Box::new(self)
    }

    #[inline]
    fn children(mut self, children: Vec<Box<dyn HtmlNode>>) -> Box<dyn HtmlNode>
    where
        Self: 'static + Send + Sync + Clone,
    {
        let dom_nodes = children
            .into_iter()
            .map(|node| node.get_dom_node())
            .collect();
        unsafe {
            self.get_dom_element_mut().set_children(dom_nodes);
        }
        Box::new(self)
    }
}
