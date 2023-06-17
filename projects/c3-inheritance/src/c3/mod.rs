mod algorithm;

use crate::{C3Object, LinearizeResult};
use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign},
};

#[derive(Copy, Clone, Debug)]
pub struct C3Progress {
    python: bool,
}

/// The inheritance graph.
#[derive(Clone, Debug, Default)]
pub struct InheritGraph {
    base: BTreeMap<String, C3Class>,
}

impl<T> AddAssign<T> for InheritGraph
where
    T: C3Object,
{
    fn add_assign(&mut self, rhs: T) {
        let class = rhs.as_class();
        self.base.insert(class.name.clone(), class);
    }
}

#[derive(Clone, Debug)]
pub struct C3Class {
    name: String,
    base: Vec<VirtualInherit>,
}

impl C3Class {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: name.into(), base: vec![] }
    }
    pub fn with_inherit<T>(mut self, name: T) -> Self
    where
        T: Into<String>,
    {
        self.base.push(VirtualInherit { class: name.into(), is_virtual: true });
        self
    }
    pub fn with_virtual_inherit<T>(mut self, name: T) -> Self
    where
        T: Into<String>,
    {
        self.base.push(VirtualInherit { class: name.into(), is_virtual: true });
        self
    }
}

impl AddAssign<VirtualInherit> for C3Class {
    fn add_assign(&mut self, rhs: VirtualInherit) {
        self.base.push(rhs);
    }
}

impl Add<VirtualInherit> for C3Class {
    type Output = Self;

    fn add(mut self, rhs: VirtualInherit) -> Self::Output {
        self.base.push(rhs);
        self
    }
}

#[derive(Clone, Debug)]
pub struct VirtualInherit {
    class: String,
    is_virtual: bool,
}

pub struct C3ClassMember {
    name: String,
}
