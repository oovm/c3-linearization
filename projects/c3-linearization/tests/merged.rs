use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub enum LinearizeError {
    NotFound { base: String },
    BadHead { base: String, this: String },
    Circular { circular: Vec<String> },
}

fn c3_linearization<'a>(classes: HashMap<&'a str, Vec<&'a str>>) -> Result<BTreeMap<&'a str, Vec<&'a str>>, LinearizeError> {
    let mut linearized = BTreeMap::new();
    let mut temporary = vec![];

    for (base, parents) in classes {
        temporary.push((base, parents));
    }

    while !temporary.is_empty() {
        let mut found = false;

        for i in 0..temporary.len() {
            let (base, parents) = temporary[i].clone();
            let mut found_head = true;

            for parent in &parents {
                let mut valid = true;

                for remaining in &temporary[i + 1..] {
                    if remaining.1.contains(parent) {
                        valid = false;
                        break;
                    }
                }

                if valid && !linearized.contains_key(parent) {
                    found_head = false;
                    break;
                }
            }

            if found_head {
                linearized.insert(base, parents.clone());
                temporary.remove(i);
                found = true;
                break;
            }
        }

        if !found {
            let circular = temporary.iter().map(|(base, _)| base.to_string()).collect();
            return Err(LinearizeError::Circular { circular });
        }
    }

    Ok(linearized)
}

#[test]
fn main() {
    let mut classes = HashMap::new();
    classes.insert("O", vec![]);
    classes.insert("A", vec!["O"]);
    classes.insert("B", vec!["O"]);
    classes.insert("C", vec!["O"]);
    classes.insert("D", vec!["O"]);
    classes.insert("E", vec!["O"]);
    classes.insert("K1", vec!["A", "B", "C"]);
    classes.insert("K2", vec!["D", "B", "E"]);
    classes.insert("K3", vec!["D", "A"]);
    classes.insert("Z", vec!["K1", "K2", "K3"]);

    println!("{:#?}", c3_linearization(classes))
}
