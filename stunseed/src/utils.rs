pub mod html_builder;

pub fn create_single_element_vec<T>(element: T) -> Vec<T> {
    let mut result = Vec::with_capacity(1);
    let end = result.as_mut_ptr();
    unsafe {
        std::ptr::write(end, element);
        result.set_len(1);
        result
    }
}

pub fn is_valid_html_attribute_key(s: &str) -> bool {
    let mut chars = s.chars();
    if let Some(first_char) = chars.next() {
        if !(first_char.is_ascii_alphabetic() || first_char == '_' || first_char == ':') {
            return false;
        }
    } else {
        return false;
    }
    for c in chars {
        if !(c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ':') {
            return false;
        }
    }
    true
}

pub fn is_valid_element_name(s: &str) -> bool {
    let mut chars = s.chars();
    if let Some(first_char) = chars.next() {
        if !first_char.is_ascii_alphabetic() {
            return false;
        }
    } else {
        return false;
    }
    for c in chars {
        if !(c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ':' || c == '.') {
            return false;
        }
    }
    true
}
