//! Range query operations.
//!
//! Find all intervals that overlap with a given query range.

use crate::node::Node;
use crate::tree::{IntervalTree, InternalNode};

impl<T> IntervalTree<T> {
    /// Find all intervals that overlap with the query range `[query_low, query_high]`.
    ///
    /// This is equivalent to [`find_overlapping`](IntervalTree::find_overlapping)
    /// but provided as a named range query for semantic clarity.
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
    /// let result = tree.range_query(4, 12);
    /// assert_eq!(result.len(), 3);
    /// ```
    pub fn range_query(&self, query_low: i64, query_high: i64) -> Vec<&Node<T>> {
        assert!(query_low <= query_high, "query_low must be <= query_high");
        self.find_overlapping(query_low, query_high)
    }

    /// Count the number of intervals overlapping with `[query_low, query_high]`.
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
    /// assert_eq!(tree.range_count(4, 12), 3);
    /// assert_eq!(tree.range_count(6, 9), 1);
    /// ```
    pub fn range_count(&self, query_low: i64, query_high: i64) -> usize {
        assert!(query_low <= query_high, "query_low must be <= query_high");
        let mut count = 0;
        if let Some(ref root) = self.root {
            Self::count_overlapping(root, query_low, query_high, &mut count);
        }
        count
    }

    fn count_overlapping(node: &InternalNode<T>, low: i64, high: i64, count: &mut usize) {
        if let Some(ref left) = node.left {
            if left.max() >= low {
                Self::count_overlapping(left, low, high, count);
            }
        }

        if node.data.overlaps_with(low, high) {
            *count += 1;
        }

        if node.data.low <= high {
            if let Some(ref right) = node.right {
                if right.max() >= low {
                    Self::count_overlapping(right, low, high, count);
                }
            }
        }
    }

    /// Find all intervals completely contained within `[query_low, query_high]`.
    ///
    /// An interval `[a, b]` is contained in `[c, d]` if `c <= a && b <= d`.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree_rs::IntervalTree;
    ///
    /// let mut tree = IntervalTree::new();
    /// tree.insert(2, 4, "a");
    /// tree.insert(1, 8, "b");
    /// tree.insert(3, 5, "c");
    ///
    /// let result = tree.contained_within(1, 8);
    /// assert_eq!(result.len(), 3);
    ///
    /// let result2 = tree.contained_within(2, 5);
    /// assert_eq!(result2.len(), 2);
    /// ```
    pub fn contained_within(&self, query_low: i64, query_high: i64) -> Vec<&Node<T>> {
        assert!(query_low <= query_high, "query_low must be <= query_high");
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            Self::search_contained(root, query_low, query_high, &mut result);
        }
        result
    }

    fn search_contained<'a>(
        node: &'a InternalNode<T>,
        query_low: i64,
        query_high: i64,
        result: &mut Vec<&'a Node<T>>,
    ) {
        if let Some(ref left) = node.left {
            Self::search_contained(left, query_low, query_high, result);
        }

        if node.data.low >= query_low && node.data.high <= query_high {
            result.push(&node.data);
        }

        if let Some(ref right) = node.right {
            Self::search_contained(right, query_low, query_high, result);
        }
    }

    /// Find all intervals that completely contain the given range `[inner_low, inner_high]`.
    ///
    /// An interval `[a, b]` contains `[c, d]` if `a <= c && d <= b`.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree_rs::IntervalTree;
    ///
    /// let mut tree = IntervalTree::new();
    /// tree.insert(1, 10, "big");
    /// tree.insert(3, 7, "medium");
    /// tree.insert(4, 6, "small");
    ///
    /// let result = tree.containing(4, 6);
    /// assert_eq!(result.len(), 3);
    /// ```
    pub fn containing(&self, inner_low: i64, inner_high: i64) -> Vec<&Node<T>> {
        assert!(inner_low <= inner_high, "inner_low must be <= inner_high");
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            Self::search_containing(root, inner_low, inner_high, &mut result);
        }
        result
    }

    fn search_containing<'a>(
        node: &'a InternalNode<T>,
        inner_low: i64,
        inner_high: i64,
        result: &mut Vec<&'a Node<T>>,
    ) {
        if let Some(ref left) = node.left {
            Self::search_containing(left, inner_low, inner_high, result);
        }

        if node.data.low <= inner_low && node.data.high >= inner_high {
            result.push(&node.data);
        }

        if let Some(ref right) = node.right {
            Self::search_containing(right, inner_low, inner_high, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_query_basic() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(3, 8, "b");
        tree.insert(10, 15, "c");

        let result = tree.range_query(4, 12);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_range_query_exact() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");

        let result = tree.range_query(1, 5);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_range_query_no_overlap() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(10, 15, "b");

        assert!(tree.range_query(6, 9).is_empty());
    }

    #[test]
    fn test_range_count() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(3, 8, "b");
        tree.insert(10, 15, "c");

        assert_eq!(tree.range_count(4, 12), 3);
        assert_eq!(tree.range_count(6, 9), 1);
    }

    #[test]
    fn test_contained_within() {
        let mut tree = IntervalTree::new();
        tree.insert(2, 4, "a");
        tree.insert(1, 8, "b");
        tree.insert(3, 5, "c");

        assert_eq!(tree.contained_within(1, 8).len(), 3);
    }

    #[test]
    fn test_contained_within_strict() {
        let mut tree = IntervalTree::new();
        tree.insert(2, 4, "a");
        tree.insert(1, 8, "b");
        tree.insert(3, 5, "c");

        assert_eq!(tree.contained_within(2, 5).len(), 2);
    }

    #[test]
    fn test_containing() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 10, "big");
        tree.insert(3, 7, "medium");
        tree.insert(4, 6, "small");

        assert_eq!(tree.containing(4, 6).len(), 3);
    }

    #[test]
    fn test_containing_none() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");

        assert!(tree.containing(0, 10).is_empty());
    }
}
