use crate::{AnElement, BiPaGraph, Elements, Elt, MetaData, Poset};

use std::collections::{HashMap, HashSet};
/// A representation of a poset encoded as a directed bipartite graph.
#[derive(PartialEq, Debug)]
pub struct PosetG {
    pub md: MetaData,
    pub g: BiPaGraph,
}

impl PosetG {
    pub fn new(g: &BiPaGraph) -> PosetG {
        PosetG {
            md: MetaData::new(g.keys().len()),
            g: g.clone(),
        }
    }
}

// TODO: Computing bot/top when minimals/maximals are known is very easy. Can do that generically?
impl Poset for PosetG {
    fn elements(&self) -> Box<dyn Iterator<Item = AnElement>> {
        Box::new(0..self.md.n)
    }

    fn leq(&self, x: AnElement, y: AnElement) -> bool {
        self.g.get(&x).unwrap().contains(&y)
    }
    fn find_bot(&mut self) {
        self.md.bot = Some(match self.g.iter().find(|(_, s)| s.len() == self.md.n) {
            Some((&i, _)) => Elt::A(i),
            None => Elt::NotPresent,
        })
    }

    fn find_top(&mut self) {
        self.find_maximals();
        self.md.top = Some(match self.md.maximals.as_ref().unwrap().len() {
            1 => Elt::A(*self.md.maximals.as_ref().unwrap().iter().next().unwrap()),
            _ => Elt::NotPresent,
        })
    }

    fn find_minimals(&mut self) {
        let non_minimals: Elements = self
            .g
            .iter()
            .map(|(i, s)| {
                let mut s_rem_i = s.clone();
                s_rem_i.remove(i);
                s_rem_i
            })
            .fold(HashSet::new(), |mut a, s| {
                a.extend(s);
                a
            });
        self.md.minimals = Some(
            (0..self.md.n)
                .filter(|i| !non_minimals.contains(i))
                .collect(),
        )
    }

    fn find_maximals(&mut self) {
        self.md.maximals = Some(
            (0..self.md.n)
                .filter(|i| self.g.get(i).unwrap().len() == 1)
                .collect(),
        )
    }

    fn op(&self) -> Self {
        let mut g: BiPaGraph = HashMap::new();
        for i in 0..self.md.n {
            let s: HashSet<_> = (0..self.md.n)
                .filter(|j| self.g.get(j).unwrap().contains(&i))
                .collect();
            g.insert(i, s);
        }
        Self::new(&g)
    }

    fn adjoin_bot(&mut self) {
        let n = self.md.n;
        let new_bot: AnElement = n;
        self.g.insert(n, (0..=n).collect());
        self.md.bot = Some(Elt::A(new_bot));
        self.md.minimals = Some(vec![new_bot].iter().cloned().collect());
        self.md.n += 1;
    }

    fn adjoin_top(&mut self) {
        let n = self.md.n;
        let new_top: AnElement = n;
        self.g.values_mut().for_each(|s| {
            s.insert(new_top);
        });
        self.g.insert(n, vec![n].iter().cloned().collect());
        self.md.top = Some(Elt::A(new_top));
        self.md.maximals = Some(vec![new_top].iter().cloned().collect());
        self.md.n += 1;
    }

    fn new_chain(n: usize) -> PosetG {
        let mut g: BiPaGraph = HashMap::new();
        for i in 0..n {
            let s: Elements = (i..n).collect();
            g.insert(i, s);
        }
        PosetG::new(&g)
    }

    fn new_antichain(n: usize) -> PosetG {
        let g: BiPaGraph = (0..n)
            .map(|i| {
                let mut s: Elements = HashSet::new();
                s.insert(i);
                (i, s)
            })
            .collect();
        Self::new(&g)
    }

    fn sub(&self, s_0: &Elements) -> Self {
        let g: BiPaGraph = s_0
            .iter()
            .map(|i| {
                (
                    *i,
                    self.g.get(i).unwrap().difference(s_0).cloned().collect(),
                )
            })
            .collect();

        Self::new(&g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_new_chain() {
        let s_0: HashSet<usize> = vec![0, 1, 2].iter().cloned().collect();
        let s_1: HashSet<usize> = vec![1, 2].iter().cloned().collect();
        let s_2: HashSet<usize> = vec![2].iter().cloned().collect();
        let mut g: BiPaGraph = HashMap::new();
        g.insert(0, s_0);
        g.insert(1, s_1);
        g.insert(2, s_2);

        assert_eq!(PosetG::new_chain(3), PosetG::new(&g))
    }

    #[test]
    fn test_new_antichain() {
        let s_0: HashSet<usize> = vec![0].iter().cloned().collect();
        let s_1: HashSet<usize> = vec![1].iter().cloned().collect();
        let s_2: HashSet<usize> = vec![2].iter().cloned().collect();
        let mut g: BiPaGraph = HashMap::new();
        g.insert(0, s_0);
        g.insert(1, s_1);
        g.insert(2, s_2);

        assert_eq!(PosetG::new_antichain(3), PosetG::new(&g))
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
        let s_0: HashSet<usize> = vec![0, 1, 2].iter().cloned().collect();
        let s_1 = vec![1].iter().cloned().collect();
        let s_2 = vec![2].iter().cloned().collect();
        let mut g = HashMap::new();
        g.insert(0, s_0);
        g.insert(1, s_1);
        g.insert(2, s_2);

        let mut vee = PosetG::new(&g);

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

        let s_0: HashSet<usize> = vec![0].iter().cloned().collect();
        let s_1 = vec![1, 0].iter().cloned().collect();
        let s_2 = vec![2, 0].iter().cloned().collect();
        let mut g = HashMap::new();
        g.insert(0, s_0);
        g.insert(1, s_1);
        g.insert(2, s_2);
        let vee_op = PosetG::new(&g);
        assert_eq!(vee.op(), vee_op);
    }
}
