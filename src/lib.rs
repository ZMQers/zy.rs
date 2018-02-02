extern crate zyre_binding;

use std::borrow::Cow;
use std::ffi::{CStr, CString};
use zyre_binding::{zyre_destroy, zyre_name, zyre_new, zyre_t, zyre_uuid};

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

    pub fn name(&self) -> Cow<str> {
        let name = unsafe {
            let c_name = zyre_name(self.zyre_node);
            CStr::from_ptr(c_name)
        };
        return name.to_string_lossy();
    }

    pub fn uuid(&self) -> Cow<str> {
        let uuid = unsafe {
            let c_uuid = zyre_uuid(self.zyre_node);
            CStr::from_ptr(c_uuid)
        };
        return uuid.to_string_lossy();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let name = "Some Node";
        let node = Node::new(name).unwrap();
        assert_eq!(node.name(), name);
    }

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

    #[test]
    fn test_uuid() {
        let node = Node::new("test node").unwrap();
        let uuid = node.uuid();
        assert_eq!(uuid.len(), 32);
    }
}
