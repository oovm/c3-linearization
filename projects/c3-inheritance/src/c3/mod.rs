mod algorithm;

use crate::{C3Object, LinearizeError, LinearizeResult};
use std::{
    collections::BTreeMap,
    fmt::Debug,
    ops::{Add, AddAssign},
};

/// The inheritance graph.
#[derive(Clone, Debug, Default)]
pub struct InheritGraph {
    base: BTreeMap<String, C3Class>,
}

#[derive(Debug)]
pub struct InheritLinearized<'a> {
    graph: &'a InheritGraph,
    maps: BTreeMap<&'a str, Vec<&'a str>>,
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

impl<'a> InheritLinearized<'a> {
    pub fn get(&self, name: &str) -> LinearizeResult<&[&str]> {
        self.maps.get(name).map(|v| &v[..]).ok_or_else(|| LinearizeError::NotFound { base: name.to_string() })
    }
}

/// A class.
#[derive(Clone, Debug)]
pub struct C3Class {
    name: String,
    base: Vec<VirtualInherit>,
}

impl C3Class {
    /// Create a new class.
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: name.into(), base: vec![] }
    }
    /// Add a base class.
    pub fn with_inherit<T>(mut self, name: T) -> Self
    where
        T: Into<String>,
    {
        self.base.push(VirtualInherit { class: name.into(), is_virtual: true });
        self
    }
    /// Add a virtual base class.
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

/// A trait for objects that can be used in the C3 algorithm.
#[derive(Clone, Debug)]
pub struct C3ClassMember {
    name: String,
    overloads: usize,
}

pub enum MemberKind {
    /// nothing
    Inherit,
    /// virtual method,
    Virtual,
    /// override method,
    Override,
    /// final method
    Final,
}
