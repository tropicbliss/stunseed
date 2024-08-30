pub mod builder;
pub mod constify;
pub mod custom_element;
pub mod include_html;
pub mod text;

use crate::{
    html::HtmlNode,
    utils::{self, html_builder::html_builder},
};
use ammonia::clean;
use stunseed_derive::HtmlElement;

include!("generated/gen_elements.rs");

impl Html {
    pub fn build(self) -> String {
        clean(&self.build_unsanitized())
    }

    pub fn build_unsanitized(self) -> String {
        html_builder(self.inner)
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
