extern crate self as stunseed;

pub mod ast;
pub mod attributes;
pub mod elements;
pub mod html;
mod utils;

pub use stunseed_derive::HtmlElement;

pub mod prelude {
    pub use crate::attributes::*;
    pub use crate::elements::*;
    pub use crate::html::*;
}
