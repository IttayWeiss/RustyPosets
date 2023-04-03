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

fn calc(isbn: &str) -> bool {
    if isbn.len() != 10 || (isbn.contains("X") && !isbn.ends_with('X')) {
        return false;
    }
    isbn.chars()
        .enumerate()
        .map(|(i, d)| {
            (i + 1)
                * (match d {
                    'X' => 10,
                    _ => d.to_digit(10).unwrap() as usize,
                })
        })
        .sum::<usize>()
        % 11
        == 0
}

#[test]
fn te() {
    println!("{}", calc("123456789X"));
}
