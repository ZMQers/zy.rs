extern crate zyre_binding;

use std::ffi::CString;
use zyre_binding::{zyre_destroy, zyre_new, zyre_t};

pub struct Node {
    zyre_node: *mut zyre_t,
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe {
            zyre_destroy(&mut self.zyre_node);
        }
    }
}

impl Node {
    pub fn new(name: &str) -> Result<Node, std::ffi::NulError> {
        let c_name = match CString::new(name) {
            Ok(cstr) => cstr,
            Err(e) => return Err(e),
        };
        let zyre_node = unsafe { zyre_new(c_name.as_ptr()) };
        let node = Node { zyre_node };
        return Ok(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        match Node::new("test node") {
            Ok(_) => (),
            Err(e) => panic!("Unexpected error creating node: {:?}", e),
        }
    }

    #[test]
    fn test_new_with_null() {
        match Node::new("test\0node") {
            Ok(_) => panic!("Didn't get expected error due to null in name"),
            Err(_) => (),
        }
    }
}
