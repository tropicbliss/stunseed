pub mod attribute_types;

#[macro_export]
macro_rules! declare_kv_attribute {
    ($name:ident, $canonical:expr, $doc:expr) => {
        ::paste::paste! {
            pub trait [<$name:camel>]: $crate::html::HtmlElement {
                #[doc = $doc]
                #[inline]
                fn [<$name:lower>]<T>(mut self: Box<Self>, value: T) -> Box<Self>
                where
                    T: Into<::smartstring::alias::String>,
                {
                    self.get_dom_element_mut()
                        .insert_attribute($canonical, $crate::ast::AttributeValue::KeyValuePair(value.into()));
                    self
                }
            }
        }
    };
}

#[macro_export]
macro_rules! declare_bool_attribute {
    ($name:ident, $canonical:expr, $doc:expr) => {
        ::paste::paste! {
            pub trait [<$name:camel>]: $crate::html::HtmlElement {
                #[doc = $doc]
                #[inline]
                fn [<$name:lower>](mut self: Box<Self>, value: bool) -> Box<Self> {
                    self.get_dom_element_mut()
                        .insert_attribute($canonical, $crate::ast::AttributeValue::BooleanAttribute(value));
                    self
                }
            }
        }
    };
}

#[macro_export]
macro_rules! declare_custom_attribute {
    ($name:ident, $canonical:expr, $doc:expr, $ty:ty) => {
        ::paste::paste! {
            pub trait [<$name:camel>]: $crate::html::HtmlElement {
                #[doc = $doc]
                #[inline]
                fn [<$name:lower>](mut self: Box<Self>, value: $ty) -> Box<Self> {
                    self.get_dom_element_mut()
                        .insert_attribute($canonical, $crate::ast::AttributeValue::KeyValuePair(value.to_string().into()));
                    self
                }
            }
        }
    };
}

#[macro_export]
macro_rules! declare_int_attribute {
    ($name:ident, $canonical:expr, $doc:expr) => {
        $crate::declare_custom_attribute!($name, $canonical, $doc, i64);
    };
}

#[macro_export]
macro_rules! declare_float_attribute {
    ($name:ident, $canonical:expr, $doc:expr) => {
        $crate::declare_custom_attribute!($name, $canonical, $doc, f64);
    };
}

include!("generated/gen_attributes.rs");
