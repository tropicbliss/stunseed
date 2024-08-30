use std::borrow::Cow;

use crate::ast::{AttributeValue, DomElement, DomNode};

pub fn html_builder(dom: DomElement, clean: bool) -> String {
    debug_assert_eq!(dom.get_name(), "html");
    let mut result = String::from("<!DOCTYPE html>");
    let html = stacker::maybe_grow(32 * 1024, 1024 * 1024, || parse_element(dom, clean));
    result.push_str(&html);
    result
}

fn parse_element(
    DomElement {
        name,
        attributes,
        children,
    }: DomElement,
    clean: bool,
) -> String {
    let mut result = String::with_capacity(70);
    result.push('<');
    result.push_str(&name);
    for (key, value) in attributes {
        match value {
            AttributeValue::BooleanAttribute(b) if b => {
                result.push(' ');
                result.push_str(&key);
            }
            AttributeValue::KeyValuePair(s) => {
                result.push(' ');
                result.push_str(&key);
                result.push_str("=\"");
                let s = escape_builder(s, clean);
                result.push_str(&s);
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
                    result.push_str(&parse_element(elem, clean));
                }
                DomNode::TextNode(text) => {
                    if last_elem_is_text {
                        result.push(' ');
                    }
                    let text = escape_builder(text, clean);
                    result.push_str(&text);
                    last_elem_is_text = true;
                }
            }
        }
        result.push_str("</");
        result.push_str(&name);
        result.push('>');
    }
    result.shrink_to_fit();
    result
}

fn escape_builder<T>(input: T, clean: bool) -> Cow<'static, str>
where
    T: Into<Cow<'static, str>>,
{
    if clean {
        let escaped = escape_text(&input.into());
        Cow::Owned(escaped)
    } else {
        input.into()
    }
}

fn escape_text(input: &str) -> String {
    let mut result = String::new();
    let mut last = 0;
    for (index, byte) in input.bytes().enumerate() {
        const MIN_CHAR: u8 = b'"';
        const MAX_CHAR: u8 = b'>';
        const TABLE: [Option<&&str>; (MAX_CHAR - MIN_CHAR + 1) as usize] = {
            let mut table = [None; (MAX_CHAR - MIN_CHAR + 1) as usize];
            table[(b'<' - MIN_CHAR) as usize] = Some(&"&lt;");
            table[(b'>' - MIN_CHAR) as usize] = Some(&"&gt;");
            table[(b'&' - MIN_CHAR) as usize] = Some(&"&amp;");
            table[(b'"' - MIN_CHAR) as usize] = Some(&"&quot;");
            table[(b'\'' - MIN_CHAR) as usize] = Some(&"&#x27;");
            table
        };

        let escaped = match byte {
            MIN_CHAR..=MAX_CHAR => TABLE[(byte - MIN_CHAR) as usize],
            _ => None,
        };
        if let Some(escaped) = escaped {
            result.push_str(&input[last..index]);
            result.push_str(&escaped);
            last = index + 1;
        }
    }
    result.push_str(&input[last..]);
    result
}
