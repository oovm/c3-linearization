use super::*;
use crate::LinearizeError;

impl InheritGraph {
    pub fn linearize(&self) -> LinearizeResult<Vec<&str>> {
        c3_linearization(&self.as_map())
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

fn c3_linearization<'a>(classes: &HashMap<&'a str, Vec<&'a str>>) -> LinearizeResult<Vec<&'a str>> {
    let mut result: Vec<&'a str> = Vec::new();
    let mut seen: HashSet<&'a str> = HashSet::new();

    fn merge<'a>(
        class: &'a str,
        classes: &HashMap<&'a str, Vec<&'a str>>,
        result: &mut Vec<&'a str>,
        seen: &mut HashSet<&'a str>,
    ) -> LinearizeResult<()> {
        if seen.contains(class) {
            return Err(LinearizeError::Circular);
        }

        seen.insert(class);

        if let Some(linearization) = classes.get(class) {
            for &base in linearization {
                merge(base, classes, result, seen)?;
            }
        }

        if !result.contains(&class) {
            result.push(class);
        }

        seen.remove(class);

        Ok(())
    }

    for class in classes.keys() {
        merge(class, classes, &mut result, &mut seen)?;
    }

    Ok(result)
}
