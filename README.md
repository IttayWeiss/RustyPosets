# RustyPosets (Work in Progress!!)
Rust code for manipulating finite partially ordered sets.

## General aim
Finite posets are combinatorially intricate structures. They can be represented computationally in different way, and the chosen representation affects,
some times quite strongly, the computational complexity of algorithms that compute, create, or manipulate posets. The aim of this repository is 
to provide code to support different representations of finite posets, together with implementations of algorithms for each form of presentation, and
functionality to convert between the different forms. 

Broadly speaking, there are three (somehwat overlapping) activities related to posets.

### Poset creation
There are numerous classes of posets that are described by one, or more, simple parameters. For instance, a chain is a poset in which all elements are comparable. A chain is determined (up to isomorphism) by its number of elements. Similarly, an anti-chain (where no two distinct elements are comparable), is also characterised by its number of elements. Other classes include various free constructions (e.g., the free distributive lattice on $n$ generators). Another way to form new posets is via categorical constructions such as limits and colimits, which include the product and coproduct (aka disjoint sum) of posets. The activity of creation of posets refers to a situation where a new poset is formed, either as "a chain with $3$ elements" or "the product of these two already existing posets". 

### Poset manipulation
A given poset can give rise to other posets by directly manipulating it. For instance, forming subposets of a given poset, or adjoining a bottom or top (or both) element to an existing poset.

### Poset computations
A poset has various invariants, such as whether it has a bottom element, the number of its minimal elements, its width, its Mobius function, etc. 

## More specific aim
By exhibiting functionality across different representations for all three main activities revolving around posets, we aim to provide:
- A playground for working with posets in an exploratory fashion.
- A robust framework to test algorithms that rely on posets. 
- A reliable computational engine to be used in applications.

## Related repositories and other resources
This list is not exhaustive! 

- [FinitePosets](https://github.com/jmichel7/FinitePosets.jl) - providing some poset functionality for Julia. 

- [Macaulay2](http://www2.macaulay2.com/Macaulay2/doc/Macaulay2-1.19.1/share/doc/Macaulay2/Posets/html/index.html) a package providing poset functionality for algebra and geometry (journal [publication](https://msp.org/jsag/2015/7-1/jsag-v7-n1-p02-s.pdf) article).
