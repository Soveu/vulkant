#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![allow(unsafe_op_in_unsafe_fn)] // why???

include!(concat!(
    env!("OUT_DIR"),
    "/bindings.rs",
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry() {
        let maybe = unsafe { vkGetInstanceProcAddr(
            core::ptr::null_mut(),
            c"vkCreateInstance".as_ptr(),
        ) };
        assert!(maybe.is_some());
    }
}
