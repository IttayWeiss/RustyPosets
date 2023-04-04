use crate::{Elt, Hasse, MetaData, Poset};

use std::collections::{HashMap, HashSet};

/// A representation of a poset encoded as a Hasse diagram.
#[derive(Debug, PartialEq)]
pub struct PosetH {
    pub md: MetaData,
    pub h: Hasse,
}

impl PosetH {
    pub fn new(h: &Hasse) -> PosetH {
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

    fn new_chain(n: usize) -> Self {
        todo!();
    }

    fn new_antichain(n: usize) -> Self {
        todo!();
    }

    fn adjoin_bot(&mut self) {
        todo!();
    }
    fn adjoin_top(&mut self) {
        todo!();
    }
}
