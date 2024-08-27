use crate::ast::{AttributeValue, DomElement, DomNode};

pub fn html_builder(dom: DomElement) -> String {
    debug_assert_eq!(dom.get_name(), "html");
    let mut result = String::from("<!DOCTYPE html>");
    let html = stacker::maybe_grow(32 * 1024, 1024 * 1024, || parse_element(&dom));
    result.push_str(&html);
    result
}

fn parse_element(elem: &DomElement) -> String {
    let name = elem.get_name();
    let attributes = elem.get_attributes();
    let children = elem.get_children();
    let mut result = String::with_capacity(70);
    result.push('<');
    result.push_str(name);
    for (key, value) in attributes {
        match value {
            AttributeValue::BooleanAttribute(b) if *b => {
                result.push(' ');
                result.push_str(key);
            }
            AttributeValue::KeyValuePair(s) => {
                result.push(' ');
                result.push_str(key);
                result.push_str("=\"");
                result.push_str(s);
                result.push('"');
            }
            _ => {}
        }
    }
    result.push('>');
    if let Some(children) = children {
        let mut last_elem_is_text = false;
        for node in children {
            match node {
                DomNode::Element(elem) => {
                    last_elem_is_text = false;
                    result.push_str(&parse_element(elem));
                }
                DomNode::TextNode(text) => {
                    if last_elem_is_text {
                        result.push(' ');
                    }
                    result.push_str(&text);
                    last_elem_is_text = true;
                }
            }
        }
        result.push_str("</");
        result.push_str(name);
        result.push('>');
    }
    result.shrink_to_fit();
    result
}
