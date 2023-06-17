use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use std::sync::Arc;
use crate::C3Object;

mod algorithm;

#[derive(Copy, Clone, Debug)]
pub struct C3 {
    reverse: bool,
    python: bool,
}


impl Default for C3 {
    fn default() -> Self {
        Self { reverse: false, python: false }
    }
}


#[derive(Debug)]
pub struct C3Class {
    name: String,
    base: Vec<VirtualClass>,
}

pub struct VirtualClass {
    is_virtual: bool,
    base: Arc<C3Class>
}


impl C3Class {
    pub fn new<S>(name: S) -> Self where S: ToString {
        Self {
            name: name.to_string(),
            is_virtual: false,
            base: vec![],
        }
    }
    pub fn set_virtual(&mut self, is_virtual: bool) {
        self.is_virtual = is_virtual;
    }
    pub fn get_virtual(&self) -> bool {
        self.is_virtual
    }
    pub fn with_virtual(mut self, is_virtual: bool) -> Self {
        self.is_virtual = is_virtual;
        self
    }
    pub fn add_inherit(&mut self, rhs: Arc<Self>) {
        self.base.push(rhs);
    }
    pub fn with_inherit(mut self, rhs: Arc<Self>) -> Self {
        self.base.push(rhs);
        self
    }
}


pub struct C3ClassInherit {
    class: String,
    is_virtual: bool,
}

pub struct C3ClassMember {
    name: String,
}

