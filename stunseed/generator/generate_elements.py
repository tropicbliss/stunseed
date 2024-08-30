import json
import os

def build_int_attribute(function_name, doc, canonical):
    return f"""
/// {doc}
    #[inline]
    pub fn {function_name}(mut self: Box<Self>, value: i64) -> Box<Self> {{
        let mut buffer = itoa::Buffer::new();
        let value = buffer.format(value).to_string();
        self.inner.insert_attribute(
            "{canonical}",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }}
"""

def build_string_attribute(function_name, doc, canonical):
    return f"""
/// {doc}
    #[inline]
    pub fn {function_name}<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {{
        self.inner.insert_attribute(
            "{canonical}",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }}
"""

def build_float_attribute(function_name, doc, canonical):
    return f"""
/// {doc}
    #[inline]
    pub fn {function_name}(mut self: Box<Self>, value: f64) -> Box<Self> {{
        let mut buffer = ryu::Buffer::new();
        let value = buffer.format(value).to_string();
        self.inner.insert_attribute(
            "{canonical}",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }}
"""

def build_bool_attribute(function_name, doc, canonical):
    return f"""
/// {doc}
    #[inline]
    pub fn {function_name}(mut self: Box<Self>, value: bool) -> Box<Self> {{
        self.inner.insert_attribute(
            "{canonical}",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }}
"""

def build(struct_name, element_name, is_void, contains_global, url_link):
    result = ""
    result += f"""
#[derive(Clone, HtmlElement)]
pub struct {struct_name} {{
    inner: crate::ast::DomElement,
}}

/// The HTML `<{element_name}>` element
///
/// {url_link}
pub fn {element_name}() -> Box<{struct_name}> {{
    Box::new({struct_name} {{
        inner: crate::ast::DomElement::{"new_void" if is_void else "new"}("{element_name}"),
    }})
}}\n
"""
    if not is_void and not element_name == "html":
        result += f"unsafe impl crate::html::NonVoidHtmlElement for {struct_name} {{}}"
    if contains_global:
        result += f"impl crate::attributes::ContainsGlobalAttributes for {struct_name} {{}}\n"
    return result

glo_elem_res = ""

directory_path = "data/merged/elements"
for filename in os.listdir(directory_path):
    file_path = os.path.join(directory_path, filename)
    with open(file_path, 'r') as file:
        data = json.load(file)
        glo_elem_res += build(data["struct_name"], data["tag_name"], not data["has_closing_tag"], data["has_global_attributes"], data["mdn_link"])
        glo_elem_res += f"\nimpl {data["struct_name"]} {{"
        for attribute in data["attributes"]:
            match attribute["ty"]:
                case "String":
                    glo_elem_res += build_string_attribute(attribute["field_name"], attribute["description"], attribute["name"])
                case "Bool":
                    glo_elem_res += build_bool_attribute(attribute["field_name"], attribute["description"], attribute["name"])
                case "Integer":
                    glo_elem_res += build_int_attribute(attribute["field_name"], attribute["description"], attribute["name"])
                case "Float":
                    glo_elem_res += build_float_attribute(attribute["field_name"], attribute["description"], attribute["name"])
        glo_elem_res += "}\n"

with open("../src/generated/gen_elements.rs", 'w') as file:
    file.write(glo_elem_res)