use crate::Result;
use crate::errors::Error::{NotFound, BadHead};


fn merge(sequences: Vec<Vec<String>>) -> Result<Vec<String>> {
    let mut result = vec![];
    //let mut sequences = sequences.iter().map(|s| s.slice()).collect::<Vec<_>>();
    let sequences = &mut sequences.clone();

    while sequences.len() > 0 {
        let mut found = false;
        //let mut head: String = String::new();


        for seq in sequences.clone() {
            println!("{:?}", seq);
           let head = seq[0].clone();
            // check bad head
            for s in sequences.clone() {
                if s != seq && s[1..s.len()].contains(&head) {
                    return Err(BadHead);
                }
            }
            if 0 == 0 {
                found = true;
                result.push(head.clone());
                for seq in sequences.iter_mut() {
                    match seq.iter().position(|r| (*r).clone() == head.clone()) {
                        Some(index) => { seq.remove(index); }
                        None => (),
                    }
                }
                break;
            }
        }
        // sequences = sequences.iter().filter(|&s| s.len() > 0).collect();
        if !found {
            return Err(NotFound);
        }
    }

    return Ok(result);
}


#[test]
fn test() {
    println!("{:#?}", merge(vec![vec!["B".to_string()], vec!["C".to_string(), "D".to_string()]]))
}