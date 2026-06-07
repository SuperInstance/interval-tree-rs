//! Overlap detection operations.
//!
//! Find all pairs of overlapping intervals in the tree, or check if a given
//! interval overlaps with any interval in the tree.

use crate::node::Node;
use crate::tree::{IntervalTree, InternalNode};

/// A pair of overlapping intervals (references).
#[derive(Debug, Clone)]
pub struct OverlapPair<'a, T> {
    /// The first interval (lower low endpoint).
    pub a: &'a Node<T>,
    /// The second interval.
    pub b: &'a Node<T>,
}

impl<T> IntervalTree<T> {
    /// Find all intervals in the tree that overlap with the given range `[low, high]`.
    ///
    /// Two intervals `[a, b]` and `[c, d]` overlap if `a <= d && c <= b`.
    /// Uses the augmented max values to prune the search efficiently.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree_rs::IntervalTree;
    ///
    /// let mut tree = IntervalTree::new();
    /// tree.insert(1, 5, "a");
    /// tree.insert(10, 15, "b");
    /// tree.insert(20, 25, "c");
    ///
    /// let result = tree.find_overlapping(4, 12);
    /// assert_eq!(result.len(), 2); // "a" and "b"
    /// ```
    pub fn find_overlapping(&self, low: i64, high: i64) -> Vec<&Node<T>> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            Self::search_overlapping(root, low, high, &mut result);
        }
        result
    }

    fn search_overlapping<'a>(
        node: &'a InternalNode<T>,
        low: i64,
        high: i64,
        result: &mut Vec<&'a Node<T>>,
    ) {
        // Check left subtree
        if let Some(ref left) = node.left {
            if left.max() >= low {
                Self::search_overlapping(left, low, high, result);
            }
        }

        // Check current node
        if node.data.overlaps_with(low, high) {
            result.push(&node.data);
        }

        // Check right subtree
        if node.data.low <= high {
            if let Some(ref right) = node.right {
                if right.max() >= low {
                    Self::search_overlapping(right, low, high, result);
                }
            }
        }
    }

    /// Find all pairs of overlapping intervals within the tree.
    ///
    /// Returns a vector of `OverlapPair` where each pair contains references
    /// to two intervals that overlap. Each pair appears exactly once.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree_rs::IntervalTree;
    ///
    /// let mut tree = IntervalTree::new();
    /// tree.insert(1, 5, "a");
    /// tree.insert(3, 8, "b");
    /// tree.insert(10, 15, "c");
    ///
    /// let pairs = tree.all_overlapping_pairs();
    /// assert_eq!(pairs.len(), 1); // only (a, b) overlap
    /// ```
    pub fn all_overlapping_pairs(&self) -> Vec<OverlapPair<'_, T>> {
        let intervals = self.intervals();
        let mut pairs = Vec::new();

        for i in 0..intervals.len() {
            for j in (i + 1)..intervals.len() {
                let a = intervals[i];
                let b = intervals[j];
                if a.high >= b.low {
                    pairs.push(OverlapPair { a, b });
                }
            }
        }

        pairs
    }

    /// Check if any interval in the tree overlaps with `[low, high]`.
    ///
    /// Returns early on the first match.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree_rs::IntervalTree;
    ///
    /// let mut tree = IntervalTree::new();
    /// tree.insert(1, 5, "a");
    /// tree.insert(10, 15, "b");
    ///
    /// assert!(tree.has_overlap(4, 12));
    /// assert!(!tree.has_overlap(6, 9));
    /// ```
    pub fn has_overlap(&self, low: i64, high: i64) -> bool {
        if let Some(ref root) = self.root {
            Self::check_overlap(root, low, high)
        } else {
            false
        }
    }

    fn check_overlap(node: &InternalNode<T>, low: i64, high: i64) -> bool {
        if node.data.overlaps_with(low, high) {
            return true;
        }

        if let Some(ref left) = node.left {
            if left.max() >= low && Self::check_overlap(left, low, high) {
                return true;
            }
        }

        if node.data.low <= high {
            if let Some(ref right) = node.right {
                if Self::check_overlap(right, low, high) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_overlapping_basic() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(10, 15, "b");
        tree.insert(20, 25, "c");

        let result = tree.find_overlapping(4, 12);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_find_overlapping_none() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(10, 15, "b");

        let result = tree.find_overlapping(6, 9);
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_overlapping_all() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 20, "a");
        tree.insert(5, 25, "b");
        tree.insert(10, 30, "c");

        let result = tree.find_overlapping(12, 18);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_all_overlapping_pairs() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(3, 8, "b");
        tree.insert(10, 15, "c");

        let pairs = tree.all_overlapping_pairs();
        assert_eq!(pairs.len(), 1);
    }

    #[test]
    fn test_all_overlapping_pairs_all_overlap() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 10, "a");
        tree.insert(5, 15, "b");
        tree.insert(8, 20, "c");

        let pairs = tree.all_overlapping_pairs();
        assert_eq!(pairs.len(), 3);
    }

    #[test]
    fn test_all_overlapping_pairs_none() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 3, "a");
        tree.insert(5, 7, "b");
        tree.insert(9, 11, "c");

        assert!(tree.all_overlapping_pairs().is_empty());
    }

    #[test]
    fn test_has_overlap_true() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(10, 15, "b");

        assert!(tree.has_overlap(4, 12));
    }

    #[test]
    fn test_has_overlap_false() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(10, 15, "b");

        assert!(!tree.has_overlap(6, 9));
    }
}
