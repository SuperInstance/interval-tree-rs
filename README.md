# interval-tree-rs

[![crates.io](https://img.shields.io/crates/v/interval-tree-rs.svg)](https://crates.io/crates/interval-tree-rs)

An interval tree data structure implemented in pure Rust with no external dependencies.

## Features

- **Insert/Delete** — O(log n) average insertion and removal
- **Stabbing Query** — Find all intervals containing a point
- **Range Query** — Find all intervals overlapping a range
- **Overlap Detection** — Find all pairs of overlapping intervals
- **Contained/Containing** — Find intervals within or enclosing a range
- Augmented BST with max-endpoint pruning for efficient queries

## Usage

```rust
use interval_tree_rs::IntervalTree;

let mut tree = IntervalTree::new();
tree.insert(1, 5, "a");
tree.insert(3, 8, "b");
tree.insert(10, 15, "c");

// Stabbing query
let containing = tree.stabbing_query(4);
assert_eq!(containing.len(), 2);

// Range query
let overlapping = tree.range_query(4, 12);
assert_eq!(overlapping.len(), 3);

// Delete
tree.delete(1, 5);
assert_eq!(tree.len(), 2);
```

## License

MIT OR Apache-2.0
