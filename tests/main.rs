use std::collections::HashMap;
use sub_projects::{merge, C3};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    println!("{:#?}", merge(vec![vec!["B".to_string()], vec!["C".to_string(), "D".to_string()]]))
}

#[test]
fn test2() {
    let c3 = C3::default();
    let mut h = HashMap::new();
    h.insert(String::from("A"), vec![String::from("B"), String::from("C")]);
    h.insert(String::from("B"), vec![]);
    h.insert(String::from("C"), vec![String::from("D")]);
    h.insert(String::from("D"), vec![]);
    println!("{:#?}", c3.linearize(h))
}
