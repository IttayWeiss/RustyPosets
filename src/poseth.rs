use crate::*;

/// A representation of a poset encoded as a Hasse diagram.
#[derive(Debug, PartialEq)]
pub struct PosetH {
    pub md: MetaData,
    h: HashMap<usize, HashSet<usize>>,
}

impl PosetH {
    pub fn new(h: HashMap<usize, HashSet<usize>>) -> PosetH {
        PosetH {
            md: MetaData::new(h.keys().len()),
            h: h.to_owned(),
        }
    }
}

impl Poset for PosetH {
    fn find_bot(&mut self) {}
    fn find_top(&mut self) {}

    fn find_minimals(&mut self) {
        todo!();
    }

    fn find_maximals(&mut self) {
        todo!();
    }

    fn op(&self) -> Self {
        todo!();
    }
}
