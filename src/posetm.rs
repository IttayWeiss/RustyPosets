use crate::{AnElement, BoolMatrix, Elements, Elt, MetaData, Poset};

use ::std::collections::HashSet;

/// A representation of a poset encoded as a matrix taking values in the boolean truth values.
#[derive(PartialEq, Debug)]
pub struct PosetM {
    pub md: MetaData,
    pub m: BoolMatrix,
}

impl PosetM {
    pub fn new(m: &BoolMatrix) -> Self {
        PosetM {
            md: MetaData::new(m.len()),
            m: m.to_owned(),
        }
    }
}

impl Poset for PosetM {
    fn elements(&self) -> Box<dyn Iterator<Item = AnElement>> {
        Box::new(0..self.md.n)
    }

    fn leq(&self, x: AnElement, y: AnElement) -> bool {
        self.m[x][y]
    }

    fn find_bot(&mut self) {
        self.md.bot = Some(
            match (0..self.md.n).find(|&i| (0..self.md.n).all(|j| self.m[i][j])) {
                Some(i) => Elt::A(i),
                None => Elt::NotPresent,
            },
        );
    }

    fn find_top(&mut self) {
        self.md.top = Some(
            match (0..self.md.n).find(|&j| (0..self.md.n).all(|i| self.m[i][j])) {
                Some(j) => Elt::A(j),
                None => Elt::NotPresent,
            },
        )
    }

    fn find_minimals(&mut self) {
        let minimals: HashSet<_> = (0..self.md.n)
            .filter(|&i| !(0..self.md.n).any(|j| i != j && self.m[j][i]))
            .collect();
        self.md.minimals = Some(minimals);
    }

    fn find_maximals(&mut self) {
        let maximals: HashSet<_> = (0..self.md.n)
            .filter(|&i| !(0..self.md.n).any(|j| i != j && self.m[i][j]))
            .collect();
        self.md.maximals = Some(maximals);
    }

    fn op(&self) -> PosetM {
        let mut m: BoolMatrix = Vec::with_capacity(self.md.n);
        for i in 0..self.md.n {
            m.push((0..self.md.n).map(|j| self.m[j][i]).collect())
        }
        PosetM::new(&m)
    }

    fn new_chain(n: usize) -> Self {
        let m: BoolMatrix = (0..n).map(|i| (0..n).map(|j| i <= j).collect()).collect();

        PosetM::new(&m)
    }

    fn new_antichain(n: usize) -> Self {
        let m: BoolMatrix = (0..n).map(|i| (0..n).map(|j| i == j).collect()).collect();

        PosetM::new(&m)
    }

    fn adjoin_bot(&mut self) {
        let n = self.md.n;
        for row in self.m.iter_mut() {
            row.push(false);
        }
        self.m.push(vec![true; n + 1]);
        self.md.n += 1;
        self.md.bot = Some(Elt::A(n));
        self.md.minimals = Some(vec![n].iter().cloned().collect());
    }

    fn adjoin_top(&mut self) {
        let n = self.md.n;
        for row in self.m.iter_mut() {
            row.push(true);
        }
        self.m.push(vec![false; n]);
        self.m[n].push(true);
        self.md.top = Some(Elt::A(n));
        self.md.maximals = Some(vec![n].iter().cloned().collect());
    }

    fn sub(&self, s_0: &Elements) -> Self {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_new_chain() {
        let m = vec![
            vec![true, true, true],
            vec![false, true, true],
            vec![false, false, true],
        ];
        assert_eq!(PosetM::new_chain(3), PosetM::new(&m));
    }

    #[test]
    fn test_new_antichain() {
        let m = vec![
            vec![true, false, false],
            vec![false, true, false],
            vec![false, false, true],
        ];
        assert_eq!(PosetM::new_antichain(3), PosetM::new(&m));
    }

    #[test]
    fn test_find_bot() {
        let mut p = PosetM::new_chain(3);
        p.find_bot();
        assert_eq!(p.md.bot, Some(Elt::A(0)));
    }

    #[test]
    fn test_find_top() {
        let mut p = PosetM::new_chain(3);
        p.find_top();
        assert_eq!(p.md.top, Some(Elt::A(2)));
    }

    #[test]
    fn test_find_minimals() {
        let mut p = PosetM::new_chain(3);
        p.find_minimals();
        let mut expected = HashSet::new();
        expected.insert(0);
        assert_eq!(p.md.minimals, Some(expected));

        let mut q = PosetM::new_antichain(3);
        q.find_minimals();
        let expected: HashSet<usize> = vec![0, 1, 2].iter().cloned().collect();
        assert_eq!(q.md.minimals, Some(expected));
    }

    #[test]
    fn test_find_maximals() {
        let mut p = PosetM::new_chain(3);
        p.find_maximals();
        let mut expected = HashSet::new();
        expected.insert(2);
        assert_eq!(p.md.maximals, Some(expected));

        let mut q = PosetM::new_antichain(3);
        q.find_maximals();
        let mut expected = HashSet::new();
        expected.insert(0);
        expected.insert(1);
        expected.insert(2);
        assert_eq!(q.md.maximals, Some(expected));
    }

    #[test]
    fn test_vee() {
        let m = vec![
            vec![true, true, true],
            vec![false, true, false],
            vec![false, false, true],
        ];

        let mut vee = PosetM::new(&m);

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

        let m = vec![
            vec![true, false, false],
            vec![true, true, false],
            vec![true, false, true],
        ];
        let vee_op = PosetM::new(&m);
        assert_eq!(vee.op(), vee_op);
    }
}
