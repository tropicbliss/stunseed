pub mod builder;
pub mod concat_children;
pub mod constify;
pub mod custom_element;
pub mod fragment;
pub mod include_html;
pub mod text;

pub use builder::*;
pub use constify::*;
pub use custom_element::*;
pub use fragment::*;
pub use text::*;

use crate::{
    html::HtmlNode,
    utils::{self, html_builder::html_builder},
};
use stunseed_derive::HtmlElement;

include!("generated/gen_elements.rs");

impl Html {
    pub fn build(self) -> String {
        html_builder(self.inner, true)
    }

    pub fn build_unsanitized(self) -> String {
        html_builder(self.inner, false)
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
