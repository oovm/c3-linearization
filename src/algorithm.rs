use crate::{
    errors::Error::{BadHead, Circular, NotFound},
    Result, C3,
};
use std::collections::{HashMap, HashSet};

pub fn merge(sequences: Vec<Vec<String>>) -> Result<Vec<String>> {
    let mut result = vec![];
    // let mut sequences = sequences.iter().map(|s| s.slice()).collect::<Vec<_>>();
    let sequences = &mut sequences.clone();
    while sequences.len() > 0 {
        let mut found = false;
        for seq in sequences.clone() {
            // println!("{:?}", seq);
            let head = seq[0].clone();
            // check bad head
            for s in sequences.clone() {
                if s != seq && s[1..s.len()].contains(&head) {
                    return Err(BadHead);
                }
            }
            found = true;
            result.push(head.clone());
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

impl C3 {
    pub fn linearize(&self, graph: HashMap<String, Vec<String>>) -> Result<HashMap<String, Vec<String>>> {
        let heads: Vec<_> = graph.keys().collect();
        let results = &mut HashMap::new();
        let visiting = &mut HashSet::new();
        for head in heads {
            self.solve(&graph, head, results, visiting)?;
        }
        return Ok(results.clone());
    }

    fn solve(
        &self,
        graph: &HashMap<String, Vec<String>>,
        head: &String,
        results: &mut HashMap<String, Vec<String>>,
        visiting: &mut HashSet<String>,
    ) -> Result<Vec<String>> {
        match results.get(head) {
            None => (),
            Some(s) => return Ok(s.clone()),
        }
        if visiting.contains(head) {
            return Err(Circular);
        }
        visiting.insert(head.clone());
        let mut parents = graph[head].clone();
        if parents.len() == 0 {
            let res = vec![head.clone()];
            results.insert(head.clone(), res.clone());
            return Ok(res);
        }
        if self.reverse {
            parents.reverse();
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
        results.insert(head.clone(), res.clone());
        visiting.remove(head);
        return Ok(res);
    }
}
