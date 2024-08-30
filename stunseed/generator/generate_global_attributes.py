import json

def replace_strings(s: str):
    return s.replace("\"", "\\\"")

def build_int_attribute(function_name, doc, canonical):
    doc = replace_strings(doc)
    return f"""
#[doc = "{doc}"]
    #[inline]
    fn {function_name}(mut self: Box<Self>, value: i64) -> Box<Self> {{
        let mut buffer = itoa::Buffer::new();
        let value = buffer.format(value).to_string();
        self.get_dom_element_mut().insert_attribute(
            "{canonical}",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }}
"""

def build_float_attribute(function_name, doc, canonical):
    doc = replace_strings(doc)
    return f"""
#[doc = "{doc}"]
    #[inline]
    fn {function_name}(mut self: Box<Self>, value: f64) -> Box<Self> {{
        let mut buffer = ryu::Buffer::new();
        let value = buffer.format(value).to_string();
        self.get_dom_element_mut().insert_attribute(
            "{canonical}",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }}
"""

def build_string_attribute(function_name, doc, canonical):
    doc = replace_strings(doc)
    return f"""
#[doc = "{doc}"]
    #[inline]
    fn {function_name}<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {{
        self.get_dom_element_mut().insert_attribute(
            "{canonical}",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }}
"""

def build_bool_attribute(function_name, doc, canonical):
    doc = replace_strings(doc)
    return f"""
#[doc = "{doc}"]
    #[inline]
    fn {function_name}(mut self: Box<Self>, value: bool) -> Box<Self> {{
        self.get_dom_element_mut().insert_attribute(
            "{canonical}",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }}
"""

glo_attr_res = "pub trait ContainsGlobalAttributes: crate::html::HtmlElement {"

with open("data/manual/global_attributes.json", 'r') as file:
    attributes = json.load(file)
    for attribute in attributes:
        match attribute["ty"]:
            case "String":
                glo_attr_res += build_string_attribute(attribute["field_name"], attribute["description"], attribute["name"])
            case "Bool":
                glo_attr_res += build_bool_attribute(attribute["field_name"], attribute["description"], attribute["name"])
            case "Integer":
                glo_attr_res += build_int_attribute(attribute["field_name"], attribute["description"], attribute["name"])
            case "Float":
                glo_attr_res += build_float_attribute(attribute["field_name"], attribute["description"], attribute["name"])

glo_attr_res += "}"

with open("../src/generated/gen_global_attributes.rs", 'w') as file:
    file.write(glo_attr_res)