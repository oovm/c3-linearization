use c3_inheritance::{C3Object, InheritGraph};
use std::collections::HashMap;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn basic() {
    let mut c3 = InheritGraph::default();
    c3 += "B";
    c3 += "D";
    c3 += "C".as_class().with_inherit("D");
    c3 += "A".as_class().with_inherit("B").with_inherit("C");
    println!("{:#?}", c3.linearize())
}
