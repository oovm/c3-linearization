use crate::{errors::LinearizeError::{BadHead, Circular, NotFound}, Result, C3, ClassStorage};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub fn merge<T>(sequences: Vec<Vec<T>>) -> Result<Vec<T>>
    where
        T: Clone + PartialEq,
{
    let mut result = vec![];
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


impl ClassStorage {
    pub fn linearize(&self) -> Result<HashMap<T, Vec<T>>>
        where
            T: Clone + PartialEq + Eq + Hash,
    {
        let heads: Vec<_> = graph.keys().collect();
        let results = &mut HashMap::new();
        let visiting = &mut HashSet::new();
        for head in heads {
            self.solve(&graph, head, results, visiting)?;
        }
        return Ok(results.clone());
    }

    fn solve<T>(
        &self,
        graph: &HashMap<T, Vec<T>>,
        head: &T,
        results: &mut HashMap<T, Vec<T>>,
        visiting: &mut HashSet<T>,
    ) -> Result<Vec<T>>
        where
            T: Clone + PartialEq + Eq + Hash,
    {
        if let Some(s) = results.get(head) {
            return Ok(s.clone());
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

impl C3 {
    pub fn linearize<T>(&self, graph: HashMap<T, Vec<T>>) -> Result<HashMap<T, Vec<T>>>
        where
            T: Clone + PartialEq + Eq + Hash,
    {
        let heads: Vec<_> = graph.keys().collect();
        let results = &mut HashMap::new();
        let visiting = &mut HashSet::new();
        for head in heads {
            self.solve(&graph, head, results, visiting)?;
        }
        return Ok(results.clone());
    }

    fn solve<T>(
        &self,
        graph: &HashMap<T, Vec<T>>,
        head: &T,
        results: &mut HashMap<T, Vec<T>>,
        visiting: &mut HashSet<T>,
    ) -> Result<Vec<T>>
        where
            T: Clone + PartialEq + Eq + Hash,
    {
        if let Some(s) = results.get(head) {
            return Ok(s.clone());
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
