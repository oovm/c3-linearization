use super::*;
use crate::LinearizeError;

impl InheritGraph {
    pub fn linearize(&self) -> LinearizeResult<BTreeMap<&str, Vec<&str>>> {
        c3_progress(self.as_map())
    }
    fn as_map(&self) -> BTreeMap<&str, Vec<&str>> {
        let mut map = BTreeMap::new();
        for (name, class) in self.base.iter() {
            let mut base = vec![];
            for subs in class.base.iter() {
                base.push(subs.class.as_str());
            }
            map.insert(name.as_str(), base);
        }
        return map;
    }
}

pub fn c3_progress<'a>(classes: BTreeMap<&'a str, Vec<&'a str>>) -> LinearizeResult<BTreeMap<&'a str, Vec<&'a str>>> {
    let mut results: BTreeMap<&'a str, Vec<&'a str>> = BTreeMap::new();
    let mut visiting: BTreeMap<&'a str, bool> = BTreeMap::new();

    for (head, _) in &classes {
        _linearize(&classes, head, &mut results, &mut visiting)?;
    }

    Ok(results)
}

fn merge(sequences: Vec<Vec<&str>>) -> LinearizeResult<Vec<&str>> {
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
            return Err(LinearizeError::NotFound { base: "".to_string() });
        }
    }

    Ok(result)
}

fn _linearize<'a>(
    graph: &BTreeMap<&'a str, Vec<&'a str>>,
    head: &'a str,
    results: &mut BTreeMap<&'a str, Vec<&'a str>>,
    visiting: &mut BTreeMap<&'a str, bool>,
) -> LinearizeResult<Vec<&'a str>> {
    if let Some(res) = results.get(head) {
        return Ok(res.clone());
    }
    check_circle(head, visiting)?;
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

fn check_circle(head: &str, visiting: &BTreeMap<&str, bool>) -> LinearizeResult<()> {
    let mut circular = Vec::with_capacity(visiting.len() + 1);
    circular.push(head.to_string());
    for (index, item) in visiting.keys().enumerate() {
        if head.eq(*item) {
            println!("circular: {:?}", circular);
            circular.push(head.to_string());
            return Err(LinearizeError::Circular { class: circular });
        }
        circular.push(item.to_string());
    }
    Ok(())
}
