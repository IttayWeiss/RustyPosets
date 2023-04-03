fn matrix_to_graph(p: PosetM) -> PosetG {
    let n = p.md.n;
    let g = (0..n)
        .zip((0..n).map(|i| (0..n).filter(|j| p.m[i][j])))
        .collect::<HashMap<_, _>>();

    PosetM::new(g)
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
    let h = (0..n)
        .zip((0..n).map(|i| {
            p.g.get(i)
                .iter()
                .filter(|j| !p.g.get(i).iter().any(|k| p.g.get(k).contains(j)))
        }))
        .collect::<HashMap<_, _>>();

    PosetH::new(&h);
}

fn graph_to_matrix(p: PosetG) -> PosetH {
    let n = p.md.n;
    let m = Vec::with_capacity(n);
    for i in 0..n {
        let row = (0..n).map(|j| p.m[i][j]);
        m.push(row);
    }

    PosetG::new(&m);
}
