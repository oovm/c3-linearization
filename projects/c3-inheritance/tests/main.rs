use std::collections::HashMap;
use c3_inheritance::{C3, C3Class, C3Object};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn basic() {
    let c3 = C3::default();
    let mut a = C3Class::new("F");

    let b = "B".as_class();
    let d = "D".as_class();
    let c = "C".as_class();

    input.insert('A', vec!['B', 'C']);
    input.insert('B', vec![]);
    input.insert('C', vec!['D']);
    input.insert('D', vec![]);
    println!("{:#?}", c3.linearize(input))
}
