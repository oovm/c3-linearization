use super::*;

impl<'a> InheritLinearized<'a> {
    fn new(graph: &'a InheritGraph) -> Self {
        Self { graph, maps: BTreeMap::new() }
    }
    fn insert(&mut self, name: &'a str, value: Vec<&'a str>) {
        self.maps.insert(name, value.to_vec());
    }
    /// Get the linearization for a class.
    pub fn mro(&self, name: &str) -> Option<&[&'a str]> {
        Some(self.maps.get(name)?.as_slice())
    }
    /// Check if A is a B
    pub fn is_ancestor(&self, this: &str, ancestor: &str) -> Option<bool> {
        Some(self.maps.get(this)?.contains(&parent))
    }
}

impl InheritGraph {
    /// Linearize the inheritance graph.
    pub fn linearize(&self) -> LinearizeResult<InheritLinearized> {
        let mut results = InheritLinearized::new(self);
        for (head, _) in &self.base {
            c3_linearize(self, head, &mut results, &mut BTreeMap::new())?;
        }
        Ok(results)
    }
    fn get_base(&self, name: &str) -> Option<&[VirtualInherit]> {
        Some(self.base.get(name)?.base.as_slice())
    }
    /// Check if the graph has at least one base class.
    pub fn has_base(&self, class: &str) -> Option<bool> {
        Some(!self.base.get(class)?.base.is_empty())
    }
}

fn c3_linearize<'a>(
    graph: &'a InheritGraph,
    head: &'a str,
    results: &mut InheritLinearized<'a>,
    visiting: &mut BTreeMap<&'a str, bool>,
) -> LinearizeResult<Vec<&'a str>> {
    if let Some(res) = results.mro(head) {
        return Ok(res.to_vec());
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
        let sequence = c3_linearize(graph, &parent.class, results, visiting)?;
        sequences.push(sequence);
    }

    if let Some(false) = graph.has_base(head) {
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
