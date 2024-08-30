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
