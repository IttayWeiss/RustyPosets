//! Finite Posets -- A crate for the creation, manipulation, and analysis of finite Partially Ordered SETS.
//!
//! A **poset** is a set together with an ordering $x \le y$ of its elements, where the ordering satisfies
//! - reflexivity: $x \le x$,
//! - anti-symmetry: if $x \le y$ and $y \le x$, then $x = y$,
//! - transitivity: if $x \le y$ and $y \le z$, then $x\le z$.
//!
//! The three aspects of creating, manipulating, and analysing posets are inter-related, but of rather distinct
//! different flavours. Creation refers to the representation of certain posets with a prescribed number
//! of elements, and satisfying certain properties. For instance, creating a chain (i.e., a totally ordered poset)
//! with exactly $n$ elements, or creating the free distributive lattice on $n$ elements. Manipulation of posets
//! includes the problem of adjoining a new bottom element to an existing poset, eliminating an existing
//! top element, or constraining the height of width of the poset in some universal manner. Problems of
//! forming the product or coproduct of two posets can be seen to belong both to creation and manipulation activities.
//! Finally, analysis refers to such problems as finding the top element (if it exists), computing the width
//! of the poset, or finding its covers.
//!
//! Some of these problems are hard, and very often the computational complexity of the problems
//! depends on the representation of the poset. This crate aims to provide functionality in this context
//! in a rather broad sense. In particular, the crate offers representations of posets.
//!
//! We are only concerned with finite posets, so from now on we will allow ourselves to briefly say 'poset'
//! instead of 'finite poset'.
//!
//! # Representations of (finite) posets
//! Mathemtically, a poset $P$ can be represented as a set $A$ together with a matrix $M\colon A\times A\to \{\bot, \top\}
//! taking values in the boolean truth values. In this way, the relation $x\le y$ holds if, and only if, $M(x,y)=\top$.
//! Another way to represent a post is by its Hasse diagram. Mathematically, the Hasse diagram if a
//! function $H\colon A\to \mathcal P(A)$ to the power set of $A$, where $y\in H(x)$ holds precisely
//! when $x\le y$ and no $t\in A$ exists with $x<t<y$. Finally, the poset can also be represented as a
//! directed bipartite graph $G$ whose vertices are the elements of the set $A$ and an edge from $x$ to $y$
//! exists if, and only if, $x\le y$. Mathematically, this graph $G$ is a function $A\to \mathcal P(A)$
//! where $x\le y$ holds if, and only if, $y\in G(x)$.
//!
//! We provide tools to convert between the different representations and to perform various manipulations
//! in each form.
//!
//! ## Poset generalities
//! In each representation of a poset, there is meta data and the actual poset encoding. The meta data
//! includes the size of the poset, the identity of top and bottom elements, etc. The meta data is uniform
//! across the different representations. The actual poset encoding differs, but still has one aspect in
//! common: the underlying 'set' is taken to be $\{0, 1, 2, ..., n-1\}$. The precise way this set in encoded
//! depends on the details of the presentation.

use std::collections::{HashMap, HashSet};

pub mod convertors;
pub mod posetg;
pub mod poseth;
pub mod posetm;

/// Provides variants for naming elements in a poset.
/// # Usefulness illustration
/// When instantiating a poset, its [MetaData]'s top value is set to None. This does not mean, though, that the poset
/// does not have a top element. All it means is that it was not (yet) computed. Once it is computed,
/// using [Poset::find_top], if it exists, then it is some value $0\le i < n$, and so top is set to
/// `Some(Elt::A(i))`. If it does not exist, then top is set to `Some(Elt::NotPresent)`.
/// ```
/// use fin_pos::Elt;
/// use fin_pos::posetg::PosetG;
/// use crate::fin_pos::Poset;
///
/// let mut p = PosetG::new_chain(3);
/// assert_eq!(p.md.top, None);
/// p.find_top();
/// assert_eq!(p.md.top, Some(Elt::A(2)));
/// ```

// Type aliases:
type AnElement = usize;
type Elements = HashSet<AnElement>;
type Hasse = HashMap<AnElement, Elements>;
type BoolMatrix = Vec<Vec<bool>>;
type BiPaGraph = HashMap<AnElement, Elements>;

#[derive(PartialEq, Debug, Hash, Eq)]
pub enum Elt {
    /// A wrapper for the name of the element. If one imagines that the underlying set consists of the
    /// $n$ elements $/{a_1, ..., a_n/}$, then this notation makes sense.
    A(usize),
    /// Verifying that non-existence of an element can be hard, so once the work is done, this variant
    /// signifies the element is not just unknown, but does not exist (see, e.g., [Poset::find_bot]).
    NotPresent,
}

/// This struct is part of any representation of a poset. It holds information about the poset
/// that can, albeit with difficulty, be computed from the encoded poset.
#[derive(PartialEq, Debug)]
pub struct MetaData {
    /// The size of the underlying set.
    pub n: usize,
    /// The top element $\top$, if it exists, is the unique element satisfying $x\le \top$ for all $x$.
    pub top: Option<Elt>,
    /// The bottom element $\bot$, if it exists, is the unique element satisfying $\bot \le x$ for all $y$.
    pub bot: Option<Elt>,
    /// An element $m$ is minimal if no element is less than it. This field holds the set of all minimal
    /// element (its cardinality is between $1$ and the size of the poset).
    pub minimals: Option<HashSet<usize>>,
    /// An element $M$ is maximal if no element is greater than it. This field holds the set of all
    /// maximal elements (its cardinality is between $1$ and the size of the poset).
    pub maximals: Option<HashSet<usize>>,
}

impl MetaData {
    fn new(n: usize) -> MetaData {
        MetaData {
            n,
            top: None,
            bot: None,
            minimals: None,
            maximals: None,
        }
    }
}

/// Functionality that can be performed on an existing poset.
pub trait Poset {
    /// Updates the poset's [MetaData] with information about its bottom element.
    fn find_bot(&mut self);

    /// Updates the poset's [MetaData] with information about its top element.
    fn find_top(&mut self);

    /// Updates the poset's [MetaData] with the set of minimal elements.
    fn find_minimals(&mut self);

    /// Updates the poset's [MetaData] with the set of maximal elements.
    fn find_maximals(&mut self);

    /// Returns the opposite of the poset.
    fn op(&self) -> Self;

    /// Creates a linearly ordered chain $\{a_1 < a_2 < \cdots < a_n\}$ of $n$ elements.
    fn new_chain(n: usize) -> Self;

    /// Creates an anti-chain of $n$ incomparable elements.
    fn new_antichain(n: usize) -> Self;

    /// Add a new bottom element to the poset.
    fn adjoin_bot(&mut self);

    /// Add a new top element to the poset.
    fn adjoin_top(&mut self);

    /// Creates a new corolla with n leaves and one root.
    fn new_corolla(n: usize) -> Self
    where
        Self: Sized,
    {
        let mut c_n = Self::new_antichain(n);
        c_n.adjoin_bot();
        c_n
    }

    fn sub(&self, s_0: &HashSet<usize>) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::posetg::PosetG;
    use crate::posetm::PosetM;

    #[test]
    fn test_new_corolla() {
        let n = 3;

        let mut c = PosetG::new_corolla(n);
        assert_eq!(c.md.n, n + 1);

        c.find_top();
        c.find_maximals();

        assert_eq!(c.md.top, Some(Elt::NotPresent));
        assert_ne!(c.md.bot, None);
        assert_eq!(c.md.minimals.unwrap().len(), 1);
        assert_eq!(c.md.maximals.unwrap().len(), n);

        let mut c = PosetM::new_corolla(n);
        assert_eq!(c.md.n, n + 1);

        c.find_top();
        c.find_maximals();

        assert_eq!(c.md.top, Some(Elt::NotPresent));
        assert_ne!(c.md.bot, None);
        assert_eq!(c.md.minimals.unwrap().len(), 1);
        assert_eq!(c.md.maximals.unwrap().len(), n);
    }
}
