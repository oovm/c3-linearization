use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
};

pub fn c3_linearization<'a>(
    classes: HashMap<&'a str, Vec<&'a str>>,
) -> Result<BTreeMap<&'a str, Vec<&'a str>>, Box<dyn Error>> {
    let mut results: BTreeMap<&'a str, Vec<&'a str>> = BTreeMap::new();
    let mut visiting: HashMap<&'a str, bool> = HashMap::new();

    for (head, _) in &classes {
        _linearize(&classes, head, &mut results, &mut visiting)?;
    }

    Ok(results)
}

fn merge(sequences: Vec<Vec<&str>>) -> Result<Vec<&str>, &'static str> {
    let mut result = Vec::new();
    let mut sequences = sequences.into_iter().map(|s| s.into_iter().collect::<Vec<&str>>()).collect::<Vec<_>>();

    while !sequences.is_empty() {
        let mut found = false;
        let mut head: Option<&str> = None;

        for seq in &sequences {
            head = seq.first().cloned();

            fn is_bad_head(seq: &&Vec<&str>, s: &&Vec<&str>) -> bool {
                s != seq && s[1..].contains(&seq[0])
            }

            if !sequences.iter().any(|s| is_bad_head(&seq, &s)) {
                found = true;
                if let Some(head) = head {
                    result.push(head);

                    for seq in &mut sequences {
                        if let Some(index) = seq.iter().position(|&x| x == head) {
                            seq.remove(index);
                        }
                    }

                    break;
                }
            }
        }

        sequences = sequences.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>();

        if !found {
            return Err("Cannot find C3-linearization for input".into());
        }
    }

    Ok(result)
}

fn _linearize<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    head: &'a str,
    results: &mut BTreeMap<&'a str, Vec<&'a str>>,
    visiting: &mut HashMap<&'a str, bool>,
) -> Result<Vec<&'a str>, Box<dyn Error>> {
    if let Some(res) = results.get(head) {
        return Ok(res.clone());
    }

    if visiting.contains_key(head) {
        return Err("Circular dependency found".into());
    }
    visiting.insert(head, true);

    let parents = graph.get(head).cloned().unwrap_or_else(Vec::new);

    if parents.is_empty() {
        let res = vec![head];
        results.insert(head, res.clone());
        return Ok(res);
    }

    let mut sequences = Vec::new();
    for parent in parents {
        let sequence = _linearize(graph, parent, results, visiting)?;
        sequences.push(sequence);
    }

    if let Some(true) = graph.get(head).map(|p| p.is_empty()) {
        sequences.push(vec![head]);
    }

    let res = vec![head].into_iter().chain(merge(sequences)?).collect::<Vec<_>>();
    results.insert(head, res.clone());

    visiting.remove(head);

    Ok(res)
}
