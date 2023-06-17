use crate::{
    c3::C3,
    errors::LinearizeError::{BadHead, Circular, NotFound},
    InheritGraph, Result,
};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub fn merge(mut sequences: Vec<Vec<&str>>) -> Result<Vec<&str>> {
    let mut result = vec![];
    while sequences.len() > 0 {
        let mut found = false;
        for seq in &sequences {
            // println!("{:?}", seq);
            let head = seq[0];
            // check bad head
            for s in &sequences {
                if s != seq && s[1..s.len()].contains(&head) {
                    return Err(BadHead);
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

impl InheritGraph {
    pub fn linearize(&self) -> Result<HashMap<&str, Vec<&str>>> {
        C3 { python: true }.linearize(self.as_map())
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

impl C3 {
    pub fn linearize<'a>(&self, graph: HashMap<&'a str, Vec<&'a str>>) -> Result<HashMap<&'a str, Vec<&'a str>>> {
        let heads: Vec<_> = graph.keys().collect();
        let mut results = HashMap::new();
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
        results: &mut HashMap<&'a str, Vec<&'a str>>,
        visiting: &mut HashSet<&'a str>,
    ) -> Result<Vec<&'a str>> {
        if let Some(s) = results.get(head) {
            return Ok(s.clone());
        }
        if visiting.contains(head) {
            return Err(Circular);
        }
        visiting.insert(head);
        let mut parents = graph[head].clone();
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
        res.extend(merge(sequences)?);
        results.insert(head, res.clone());
        visiting.remove(head);
        return Ok(res);
    }
}
