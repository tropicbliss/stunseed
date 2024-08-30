use std::{borrow::Cow, collections::HashMap};

#[derive(Clone)]
pub enum DomNode {
    Element(DomElement),
    TextNode(Cow<'static, str>),
}

#[derive(Clone)]
pub struct DomElement {
    pub(crate) name: &'static str,
    pub(crate) attributes: HashMap<&'static str, AttributeValue>,
    pub(crate) children: Option<Vec<DomNode>>,
}

impl DomElement {
    pub fn new(name: &'static str) -> Self {
        DomElement {
            name,
            attributes: HashMap::new(),
            children: Some(Vec::new()),
        }
    }

    pub fn new_void(name: &'static str) -> Self {
        DomElement {
            name,
            attributes: HashMap::new(),
            children: None,
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_attributes(&self) -> &HashMap<&'static str, AttributeValue> {
        &self.attributes
    }

    pub fn get_children(&self) -> &Option<Vec<DomNode>> {
        &self.children
    }

    pub fn insert_attribute(&mut self, key: &'static str, value: AttributeValue) {
        self.attributes.insert(key, value);
    }

    pub unsafe fn set_children(&mut self, children: Vec<DomNode>) {
        debug_assert!(
            self.children.is_some(),
            "cannot set children of void element"
        );
        *self.children.as_mut().unwrap_unchecked() = children;
    }
}

#[derive(Clone)]
pub enum AttributeValue {
    KeyValuePair(Cow<'static, str>),
    BooleanAttribute(bool),
}
