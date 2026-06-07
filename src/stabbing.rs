//! Stabbing query operations.
//!
//! Find all intervals containing a given point.

use crate::node::Node;
use crate::tree::{IntervalTree, InternalNode};

impl<T> IntervalTree<T> {
    /// Find all intervals that contain the given point.
    ///
    /// A point `p` is contained in interval `[low, high]` if `low <= p <= high`.
    ///
    /// Returns a vector of references to the matching nodes.
    /// Uses the augmented max values to prune the search.
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
    /// let result = tree.stabbing_query(4);
    /// assert_eq!(result.len(), 2);
    /// ```
    pub fn stabbing_query(&self, point: i64) -> Vec<&Node<T>> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            Self::stabbing_search(root, point, &mut result);
        }
        result
    }

    fn stabbing_search<'a>(
        node: &'a InternalNode<T>,
        point: i64,
        result: &mut Vec<&'a Node<T>>,
    ) {
        // Check left subtree if it could contain the point
        if let Some(ref left) = node.left {
            if left.max() >= point {
                Self::stabbing_search(left, point, result);
            }
        }

        // Check current node
        if node.data.contains_point(point) {
            result.push(&node.data);
        }

        // Check right subtree if node.low <= point
        if node.data.low <= point {
            if let Some(ref right) = node.right {
                if right.max() >= point {
                    Self::stabbing_search(right, point, result);
                }
            }
        }
    }

    /// Find all interval values that contain the given point.
    ///
    /// Convenience method that extracts the values from the stabbing query results.
    pub fn stabbing_values(&self, point: i64) -> Vec<&T> {
        self.stabbing_query(point).iter().map(|n| &n.value).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stabbing_single_match() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(10, 15, "b");

        let result = tree.stabbing_query(3);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, "a");
    }

    #[test]
    fn test_stabbing_multiple_matches() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(3, 8, "b");
        tree.insert(2, 6, "c");

        let result = tree.stabbing_query(4);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_stabbing_no_match() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 5, "a");
        tree.insert(10, 15, "b");

        let result = tree.stabbing_query(7);
        assert!(result.is_empty());
    }

    #[test]
    fn test_stabbing_boundary_low() {
        let mut tree = IntervalTree::new();
        tree.insert(5, 10, "a");

        let result = tree.stabbing_query(5);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_stabbing_boundary_high() {
        let mut tree = IntervalTree::new();
        tree.insert(5, 10, "a");

        let result = tree.stabbing_query(10);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_stabbing_empty_tree() {
        let tree: IntervalTree<i32> = IntervalTree::new();
        assert!(tree.stabbing_query(5).is_empty());
    }

    #[test]
    fn test_stabbing_point_interval() {
        let mut tree = IntervalTree::new();
        tree.insert(5, 5, "point");

        assert_eq!(tree.stabbing_query(5).len(), 1);
        assert!(tree.stabbing_query(4).is_empty());
        assert!(tree.stabbing_query(6).is_empty());
    }
}
