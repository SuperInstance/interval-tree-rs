//! # interval-tree-rs
//!
//! An interval tree data structure implemented in pure Rust with no external dependencies.
//!
//! ## Features
//!
//! - **Insert/Delete** — Add and remove intervals with associated values
//! - **Stabbing Query** — Find all intervals containing a given point
//! - **Range Query** — Find all intervals overlapping a given range
//! - **Overlap Detection** — Find all pairs of overlapping intervals
//!
//! ## Example
//!
//! ```
//! use interval_tree_rs::IntervalTree;
//!
//! let mut tree = IntervalTree::new();
//! tree.insert(1, 5, "a");
//! tree.insert(3, 8, "b");
//! tree.insert(10, 15, "c");
//!
//! // Stabbing query: which intervals contain point 4?
//! let containing = tree.stabbing_query(4);
//! assert_eq!(containing.len(), 2);
//!
//! // Range query: which intervals overlap [4, 12]?
//! let overlapping = tree.range_query(4, 12);
//! assert_eq!(overlapping.len(), 3);
//! ```

pub mod node;
pub mod tree;
pub mod stabbing;
pub mod overlap;
pub mod range;

pub use tree::IntervalTree;
pub use node::Node;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_insert_and_len() {
        let mut tree = IntervalTree::new();
        assert!(tree.is_empty());
        tree.insert(1, 5, "a");
        assert_eq!(tree.len(), 1);
        tree.insert(3, 8, "b");
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn test_delete_basic() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(3, 8, "b");

        let removed = tree.delete(1, 5);
        assert_eq!(removed, Some("a"));
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_delete_nonexistent() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");

        let removed = tree.delete(10, 20);
        assert!(removed.is_none());
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_delete_all() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(3, 8, "b");

        tree.delete(1, 5);
        tree.delete(3, 8);
        assert!(tree.is_empty());
    }

    #[test]
    fn test_stabbing_nested_intervals() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 20, "outer");
        tree.insert(5, 15, "middle");
        tree.insert(8, 12, "inner");

        let result = tree.stabbing_query(10);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_stabbing_after_delete() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(3, 8, "b");

        tree.delete(1, 5);
        let result = tree.stabbing_query(4);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, "b");
    }

    #[test]
    fn test_range_query_wide() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(10, 15, "b");
        tree.insert(20, 25, "c");

        let result = tree.range_query(0, 100);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_range_query_single_point() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");

        // Range query with a single point
        let result = tree.range_query(3, 3);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_overlapping_pairs_complex() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 10, "a");
        tree.insert(5, 15, "b");
        tree.insert(12, 20, "c");
        tree.insert(25, 30, "d");

        let pairs = tree.all_overlapping_pairs();
        // (a,b), (a,c)?? no: a=[1,10], c=[12,20] -> 10 < 12, no overlap
        // (b,c): b=[5,15], c=[12,20] -> 15 >= 12, overlap
        assert_eq!(pairs.len(), 2); // (a,b) and (b,c)
    }

    #[test]
    fn test_point_intervals() {
        let mut tree = IntervalTree::new();
        tree.insert(5, 5, "p1");
        tree.insert(5, 5, "p2"); // same point
        tree.insert(10, 10, "p3");

        let result = tree.stabbing_query(5);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_negative_intervals() {
        let mut tree = IntervalTree::new();
        tree.insert(-10, -5, "a");
        tree.insert(-3, 3, "b");
        tree.insert(5, 10, "c");

        let result = tree.stabbing_query(0);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, "b");
    }

    #[test]
    fn test_intervals_sorted_order() {
        let mut tree = IntervalTree::new();
        tree.insert(10, 15, "c");
        tree.insert(1, 5, "a");
        tree.insert(3, 8, "b");

        let intervals = tree.intervals();
        // Should be sorted by low endpoint
        assert!(intervals.windows(2).all(|w| w[0].low <= w[1].low));
    }

    #[test]
    fn test_has_overlap_edge_case() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(5, 10, "b"); // Touching at point 5

        // [1,5] and [5,10] both overlap with query [5,5]
        assert!(tree.has_overlap(5, 5));
        assert_eq!(tree.find_overlapping(5, 5).len(), 2);
    }

    #[test]
    fn test_large_tree() {
        let mut tree = IntervalTree::new();
        for i in 0..100 {
            tree.insert(i * 10, i * 10 + 5, i);
        }
        assert_eq!(tree.len(), 100);

        // Point 50 should be in interval [50, 55]
        let result = tree.stabbing_query(50);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, 5);
    }

    #[test]
    fn test_contained_within_nested() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 20, "outer");
        tree.insert(5, 15, "middle");
        tree.insert(8, 12, "inner");

        let result = tree.contained_within(1, 20);
        assert_eq!(result.len(), 3);

        let result2 = tree.contained_within(5, 15);
        assert_eq!(result2.len(), 2); // middle and inner
    }

    #[test]
    fn test_containing_nested() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 20, "outer");
        tree.insert(5, 15, "middle");
        tree.insert(8, 12, "inner");

        let result = tree.containing(8, 12);
        assert_eq!(result.len(), 3); // all three contain [8,12]
    }
}
