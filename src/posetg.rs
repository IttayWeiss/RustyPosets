use crate::*;

/// A representation of a poset encoded as a directed bipartite graph.
#[derive(PartialEq, Debug)]
pub struct PosetG {
    pub md: MetaData,
    h: HashMap<usize, HashSet<usize>>,
}

impl PosetG {
    pub fn new(h: &HashMap<usize, HashSet<usize>>) -> PosetG {
        PosetG {
            md: MetaData::new(h.keys().len()),
            h: h.clone(),
        }
    }
}

// TODO: Computing bot/top when minimals/maximals are known is very easy. Can do that generically?
impl Poset for PosetG {
    fn find_bot(&mut self) {
        self.md.bot = Some(
            match self.h.iter().find(|(_, s)| s.len() == self.md.n - 1) {
                Some((&i, _)) => Elt::A(i),
                None => Elt::NotPresent,
            },
        )
    }

    fn find_top(&mut self) {
        self.find_maximals();
        self.md.top = Some(match self.md.maximals.as_ref().unwrap().len() {
            1 => Elt::A(*self.md.maximals.as_ref().unwrap().iter().next().unwrap()),
            _ => Elt::NotPresent,
        })
    }

    fn find_minimals(&mut self) {
        let union: HashSet<usize> = self.h.values().fold(HashSet::new(), |mut a, s| {
            a.extend(s);
            a
        });
        self.md.minimals = Some((0..self.md.n).filter(|i| !union.contains(i)).collect())
    }

    fn find_maximals(&mut self) {
        self.md.maximals = Some(
            (0..self.md.n)
                .filter(|i| self.h.get(i).unwrap().is_empty())
                .collect(),
        )
    }

    fn op(&self) -> Self {
        let mut h = HashMap::new();
        for i in 0..self.md.n {
            let s: HashSet<_> = (0..self.md.n)
                .filter(|j| self.h.get(j).unwrap().contains(&i))
                .collect();
            h.insert(i, s);
        }
        PosetG::new(&h)
    }
}

impl PosetConstructors for PosetG {
    fn new_chain(n: usize) -> PosetG {
        let mut h = HashMap::new();
        for i in 0..n {
            let s: HashSet<_> = (i + 1..n).collect();
            h.insert(i, s);
        }
        PosetG::new(&h)
    }

    fn new_antichain(n: usize) -> PosetG {
        let mut h: HashMap<usize, HashSet<usize>> = HashMap::new();
        for i in 0..n {
            h.insert(i, HashSet::new());
        }
        PosetG::new(&h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_new_chain() {
        let s_0: HashSet<usize> = vec![1, 2].iter().cloned().collect();
        let s_1: HashSet<usize> = vec![2].iter().cloned().collect();
        let s_2: HashSet<usize> = vec![].iter().cloned().collect();
        let mut h = HashMap::new();
        h.insert(0, s_0);
        h.insert(1, s_1);
        h.insert(2, s_2);

        assert_eq!(PosetG::new_chain(3), PosetG::new(&h))
    }

    #[test]
    fn test_new_antichain() {
        let s = HashSet::new();
        let mut h = HashMap::new();
        h.insert(0, s.clone());
        h.insert(1, s.clone());
        h.insert(2, s);

        assert_eq!(PosetG::new_antichain(3), PosetG::new(&h))
    }

    #[test]
    fn test_find_bot() {
        let mut p = PosetG::new_chain(3);
        p.find_bot();
        assert_eq!(p.md.bot, Some(Elt::A(0)));
    }

    #[test]
    fn test_find_top() {
        let mut p = PosetG::new_chain(3);
        p.find_top();
        assert_eq!(p.md.top, Some(Elt::A(2)));
    }

    #[test]
    fn test_find_minimals() {
        let mut p = PosetG::new_chain(3);
        p.find_minimals();
        let mut expected = HashSet::new();
        expected.insert(0);
        assert_eq!(p.md.minimals, Some(expected));

        let mut q = PosetG::new_antichain(3);
        q.find_minimals();
        let expected: HashSet<usize> = vec![0, 1, 2].iter().cloned().collect();
        assert_eq!(q.md.minimals, Some(expected));
    }

    #[test]
    fn test_find_maximals() {
        let mut p = PosetG::new_chain(3);
        p.find_maximals();
        let mut expected = HashSet::new();
        expected.insert(2);
        assert_eq!(p.md.maximals, Some(expected));

        let mut q = PosetG::new_antichain(3);
        q.find_maximals();
        let mut expected = HashSet::new();
        expected.insert(0);
        expected.insert(1);
        expected.insert(2);
        assert_eq!(q.md.maximals, Some(expected));
    }

    #[test]
    fn test_vee() {
        let s_0: HashSet<usize> = vec![1, 2].iter().cloned().collect();
        let s_1 = HashSet::new();
        let s_2 = HashSet::new();
        let mut h = HashMap::new();
        h.insert(0, s_0);
        h.insert(1, s_1);
        h.insert(2, s_2);

        let mut vee = PosetG::new(&h);

        let minimals: HashSet<usize> = vec![0].iter().cloned().collect();
        let maximals: HashSet<usize> = vec![1, 2].iter().cloned().collect();
        let top = Some(Elt::NotPresent);
        let bot = Some(Elt::A(0));

        vee.find_top();
        vee.find_bot();
        vee.find_minimals();
        vee.find_maximals();
        assert_eq!(vee.md.bot, bot);
        assert_eq!(vee.md.top, top);
        assert_eq!(vee.md.minimals, Some(minimals));
        assert_eq!(vee.md.maximals, Some(maximals));

        let s_0: HashSet<usize> = HashSet::new();
        let s_1: HashSet<usize> = vec![0].iter().cloned().collect();
        let s_2: HashSet<usize> = vec![0].iter().cloned().collect();
        let mut h = HashMap::new();
        h.insert(0, s_0);
        h.insert(1, s_1);
        h.insert(2, s_2);
        let vee_op = PosetG::new(&h);
        assert_eq!(vee.op(), vee_op);
    }
}
