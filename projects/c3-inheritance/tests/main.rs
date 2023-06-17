use c3_inheritance::{C3Object, InheritGraph, LinearizeError};
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

/// <https://zh.wikipedia.org/wiki/C3线性化>
#[test]
fn wiki_1() -> Result<(), LinearizeError> {
    let mut c3 = InheritGraph::default();
    c3 += "A";
    c3 += "B";
    c3 += "C";
    c3 += "D";
    c3 += "E";
    c3 += "K1".as_class().with_inherit("A").with_inherit("B").with_inherit("C");
    c3 += "K2".as_class().with_inherit("D").with_inherit("B").with_inherit("E");
    c3 += "K3".as_class().with_inherit("D").with_inherit("A");
    c3 += "Z".as_class().with_inherit("K1").with_inherit("K2").with_inherit("K3");
    let out = c3.linearize()?;
    let z = out.get("Z").unwrap().as_slice();
    assert_eq!(z, &["Z", "K1", "K2", "K3", "D", "A", "B", "C", "E"]);
    Ok(())
}

/// <https://en.wikipedia.org/wiki/C3_linearization>
#[test]
fn wiki_2() -> Result<(), LinearizeError> {
    let mut c3 = InheritGraph::default();
    c3 += "A";
    c3 += "B";
    c3 += "C";
    c3 += "D";
    c3 += "E";
    c3 += "K1".as_class().with_inherit("C").with_inherit("A").with_inherit("B");
    c3 += "K2".as_class().with_inherit("B").with_inherit("D").with_inherit("E");
    c3 += "K3".as_class().with_inherit("A").with_inherit("D");
    c3 += "Z".as_class().with_inherit("K1").with_inherit("K3").with_inherit("K2");
    let out = c3.linearize()?;
    let z = out.get("Z").unwrap().as_slice();
    assert_eq!(z, &["Z", "K1", "C", "K3", "A", "K2", "B", "D", "E"]);
    Ok(())
}
