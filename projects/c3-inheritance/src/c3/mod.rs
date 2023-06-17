use std::collections::{BTreeMap, HashMap};
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


#[derive(Clone, Debug, Default)]
pub struct ClassStorage {
    base: BTreeMap<String, C3Class>,
}

impl<T> AddAssign<T> for ClassStorage where T: C3Object {
    fn add_assign(&mut self, rhs: T) {
        todo!()
    }
}

pub struct C3Class {
    name: String,
    base: Vec<(bool, String)>
}

impl C3Class {
    pub fn new<T>(name: T) -> Self where T: Into<String> {
        Self {
            name: name.into(),
            base: vec![]
        }
    }
}

pub struct C3ClassInherit {
    class: String,
    is_virtual: bool,
}

pub struct C3ClassMember {
    name: String,
}

