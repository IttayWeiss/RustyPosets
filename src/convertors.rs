use crate::posetg::PosetG;
use crate::poseth::PosetH;
use crate::posetm::PosetM;
use crate::{BiPaGraph, BoolMatrix, Hasse};

use std::collections::HashMap;

fn matrix_to_graph(p: PosetM) -> PosetG {
    let n = p.md.n;
    let g = (0..n)
        .zip((0..n).map(|i| (0..n).filter(|&j| p.m[i][j]).collect()))
        .collect::<HashMap<_, _>>();

    PosetG::new(&g)
}

fn matrix_to_hasse(p: PosetM) -> PosetH {
    todo!();
}

fn hasse_to_matrix(p: PosetH) -> PosetM {
    todo!();
}

fn hasse_to_graph(p: PosetH) -> PosetG {
    todo!();
}

fn graph_to_hasse(p: PosetG) -> PosetH {
    let n = p.md.n;
    let h: Hasse = (0..n)
        .zip((0..n).map(|i| {
            p.g.get(&i)
                .unwrap()
                .iter()
                .filter(|j| {
                    !p.g.get(&i)
                        .unwrap()
                        .iter()
                        .any(|k| p.g.get(k).unwrap().contains(j))
                })
                .map(|&x| x)
                .collect()
        }))
        .collect();

    PosetH::new(&h)
}

fn graph_to_matrix(p: PosetG) -> PosetM {
    let n = p.md.n;
    let mut m: BoolMatrix = Vec::with_capacity(n);
    for i in 0..n {
        let row = (0..n).map(|j| p.g.get(&i).unwrap().contains(&j)).collect();
        m.push(row);
    }

    PosetM::new(&m)
}
