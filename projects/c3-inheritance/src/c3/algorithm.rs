use super::*;

impl InheritGraph {
    pub fn linearize(&self) -> Result<BTreeMap<&str, Vec<&str>>> {
        C3Progress { python: true }.linearize(self.as_map())
    }
    fn as_map(&self) -> HashMap<&str, Vec<&str>> {
        let mut map = HashMap::new();
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

impl C3Progress {
    pub fn linearize<'a>(&self, graph: HashMap<&'a str, Vec<&'a str>>) -> Result<BTreeMap<&'a str, Vec<&'a str>>> {
        let heads: Vec<_> = graph.keys().collect();
        let mut results = BTreeMap::new();
        let mut visiting = HashSet::new();
        for head in heads {
            self.solve(&graph, head, &mut results, &mut visiting)?;
        }
        return Ok(results);
    }

    fn solve<'a>(
        &self,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        head: &'a str,
        results: &mut BTreeMap<&'a str, Vec<&'a str>>,
        visiting: &mut HashSet<&'a str>,
    ) -> Result<Vec<&'a str>> {
        if let Some(s) = results.get(head) {
            return Ok(s.clone());
        }
        if visiting.contains(head) {
            return Err(Circular);
        }
        visiting.insert(head);
        let parents = graph[head].clone();
        if parents.len() == 0 {
            let res = vec![head];
            results.insert(head, res.clone());
            return Ok(res);
        }
        let mut sequences = vec![];
        for x in parents.iter() {
            let s = self.solve(graph, &x, results, visiting)?;
            sequences.push(s)
        }
        if self.python {
            sequences.extend(vec![parents])
        }
        let mut res = vec![head.clone()];
        res.extend(merge(head, sequences)?);
        results.insert(head, res.clone());
        visiting.remove(head);
        return Ok(res);
    }
}

pub fn merge<'a>(base: &'a str, mut sequences: Vec<Vec<&'a str>>) -> Result<Vec<&'a str>> {
    let mut result = vec![];
    while sequences.len() > 0 {
        let mut found = false;
        for seq in &sequences {
            // println!("{:?}", seq);
            let head = seq[0];
            // check bad head
            for s in &sequences {
                if s != seq && s[1..s.len()].contains(&head) {
                    return Err(BadHead { base: head.to_string(), this: base.to_string() });
                }
            }
            found = true;
            result.push(head);
            for seq in sequences.iter_mut() {
                if let Some(index) = seq.iter().position(|r| r == &head) {
                    seq.remove(index);
                }
            }
            break;
        }
        sequences.retain(|x| !x.is_empty());
        if !found {
            return Err(NotFound);
        }
    }

    return Ok(result);
}
