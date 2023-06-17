use c3_inheritance::{C3Object, InheritGraph};
use std::collections::{HashMap, HashSet};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn bad_head() {
    let mut c3 = InheritGraph::default();
    c3 += "F".as_class();
    c3 += "E".as_class().with_inherit("F");
    c3 += "G".as_class().with_inherit("F").with_inherit("E");
    c3 += "D";
    println!("{:#?}", c3.linearize())
}

//  class O
//  class A extends O
//  class B extends O
//  class C extends O
//  class D extends O
//  class E extends O
//  class K1 extends A, B, C
//  class K2 extends D, B, E
//  class K3 extends D, A
//  class Z extends K1, K2, K3
#[test]
fn basic() {
    let mut c3 = InheritGraph::default();
    c3 += "object".as_class().with_inherit("Z");
    c3 += "A".as_class().with_inherit("object");
    c3 += "B".as_class().with_inherit("object");
    c3 += "C".as_class().with_inherit("object");
    c3 += "D".as_class().with_inherit("object");
    c3 += "E".as_class().with_inherit("object");
    c3 += "K1".as_class().with_inherit("A").with_inherit("B").with_inherit("C");
    c3 += "K2".as_class().with_inherit("D").with_inherit("B").with_inherit("E");
    c3 += "K3".as_class().with_inherit("D").with_inherit("A");
    c3 += "Z".as_class().with_inherit("K1").with_inherit("K2").with_inherit("K3");
    println!("{:#?}", c3.linearize().unwrap())
}
