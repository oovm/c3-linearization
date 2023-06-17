use super::*;
use crate::LinearizeError;

impl InheritGraph {
    /// Linearize the inheritance graph.
    pub fn linearize(&self) -> LinearizeResult<BTreeMap<&str, Vec<&str>>> {
        c3_progress(self)
    }
    fn get_base(&self, name: &str) -> Option<&[VirtualInherit]> {
        Some(self.base.get(name)?.base.as_slice())
    }
}

fn c3_progress<'a>(classes: &'a InheritGraph) -> LinearizeResult<BTreeMap<&'a str, Vec<&'a str>>> {
    let mut results: BTreeMap<&'a str, Vec<&'a str>> = BTreeMap::new();
    let mut visiting: BTreeMap<&'a str, bool> = BTreeMap::new();

    for (head, _) in &classes.base {
        _linearize(classes, head, &mut results, &mut visiting)?;
    }

    Ok(results)
}

fn _linearize<'a>(
    graph: &'a InheritGraph,
    head: &'a str,
    results: &mut BTreeMap<&'a str, Vec<&'a str>>,
    visiting: &mut BTreeMap<&'a str, bool>,
) -> LinearizeResult<Vec<&'a str>> {
    if let Some(res) = results.get(head) {
        return Ok(res.clone());
    }
    check_circle(head, visiting)?;
    visiting.insert(head, true);

    let parents = graph.get_base(head).unwrap();

    if parents.is_empty() {
        let res = vec![head];
        results.insert(head, res.clone());
        return Ok(res);
    }

    let mut sequences = Vec::new();
    for parent in parents {
        let sequence = _linearize(graph, &parent.class, results, visiting)?;
        sequences.push(sequence);
    }

    if let Some(true) = graph.get_base(head).map(|p| p.is_empty()) {
        sequences.push(vec![head]);
    }

    let res = vec![head].into_iter().chain(merge(sequences)?).collect::<Vec<_>>();
    results.insert(head, res.clone());

    visiting.remove(head);

    Ok(res)
}
fn merge(sequences: Vec<Vec<&str>>) -> LinearizeResult<Vec<&str>> {
    let mut result = Vec::new();
    let mut sequences = sequences.into_iter().map(|s| s.into_iter().collect::<Vec<&str>>()).collect::<Vec<_>>();

    while !sequences.is_empty() {
        let mut found = false;
        let mut head = None;

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
            return Err(LinearizeError::NotFound { base: head.unwrap().to_string() });
        }
    }

    Ok(result)
}

fn check_circle(head: &str, visiting: &BTreeMap<&str, bool>) -> LinearizeResult<()> {
    for item in visiting.keys() {
        if head.eq(*item) {
            return Err(LinearizeError::Circular { class: head.to_string() });
        }
    }
    Ok(())
}
