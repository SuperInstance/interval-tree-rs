//! Interval node representation.
//!
//! An interval `[low, high]` with an associated value, stored in the tree.

/// A single interval with an associated value.
///
/// The interval is closed: `[low, high]`, meaning both endpoints are inclusive.
///
/// # Examples
///
/// ```
/// use interval_tree_rs::Node;
///
/// let node = Node::new(1, 5, "hello");
/// assert_eq!(node.low, 1);
/// assert_eq!(node.high, 5);
/// assert_eq!(node.value, "hello");
/// ```
#[derive(Debug, Clone)]
pub struct Node<T> {
    /// The left endpoint of the interval (inclusive).
    pub low: i64,
    /// The right endpoint of the interval (inclusive).
    pub high: i64,
    /// The maximum high value in the subtree rooted at this node.
    pub max: i64,
    /// The associated value.
    pub value: T,
}

impl<T> Node<T> {
    /// Create a new interval node with the given range and value.
    ///
    /// # Panics
    ///
    /// Panics if `low > high`.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree_rs::Node;
    ///
    /// let node = Node::new(0, 10, 42);
    /// assert_eq!(node.low, 0);
    /// assert_eq!(node.high, 10);
    /// ```
    pub fn new(low: i64, high: i64, value: T) -> Self {
        assert!(low <= high, "low must be <= high (got low={}, high={})", low, high);
        Self {
            low,
            high,
            max: high,
            value,
        }
    }

    /// Check if a point is contained within this interval.
    ///
    /// Returns `true` if `point >= low && point <= high`.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree_rs::Node;
    ///
    /// let node = Node::new(2, 8, ());
    /// assert!(node.contains_point(5));
    /// assert!(node.contains_point(2));
    /// assert!(node.contains_point(8));
    /// assert!(!node.contains_point(1));
    /// assert!(!node.contains_point(9));
    /// ```
    pub fn contains_point(&self, point: i64) -> bool {
        point >= self.low && point <= self.high
    }

    /// Check if this interval overlaps with another interval `[other_low, other_high]`.
    ///
    /// Two intervals `[a, b]` and `[c, d]` overlap if `a <= d && c <= b`.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree_rs::Node;
    ///
    /// let node = Node::new(2, 8, ());
    /// assert!(node.overlaps_with(5, 10));
    /// assert!(node.overlaps_with(1, 3));
    /// assert!(!node.overlaps_with(9, 12));
    /// ```
    pub fn overlaps_with(&self, other_low: i64, other_high: i64) -> bool {
        self.low <= other_high && other_low <= self.high
    }

    /// Update the max value to be the maximum of this node's high and the given children's max values.
    pub(crate) fn update_max(&mut self, left_max: Option<i64>, right_max: Option<i64>) {
        self.max = self.high;
        if let Some(lm) = left_max {
            self.max = self.max.max(lm);
        }
        if let Some(rm) = right_max {
            self.max = self.max.max(rm);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = Node::new(1, 5, "test");
        assert_eq!(node.low, 1);
        assert_eq!(node.high, 5);
        assert_eq!(node.max, 5);
        assert_eq!(node.value, "test");
    }

    #[test]
    #[should_panic]
    fn test_invalid_range() {
        let _ = Node::new(5, 1, "bad");
    }

    #[test]
    fn test_contains_point_interior() {
        let node = Node::new(2, 8, ());
        assert!(node.contains_point(5));
    }

    #[test]
    fn test_contains_point_boundary_low() {
        let node = Node::new(2, 8, ());
        assert!(node.contains_point(2));
    }

    #[test]
    fn test_contains_point_boundary_high() {
        let node = Node::new(2, 8, ());
        assert!(node.contains_point(8));
    }

    #[test]
    fn test_contains_point_outside() {
        let node = Node::new(2, 8, ());
        assert!(!node.contains_point(1));
        assert!(!node.contains_point(9));
    }

    #[test]
    fn test_overlaps_adjacent() {
        let node = Node::new(2, 8, ());
        // Adjacent [8, 10] shares a boundary — overlapping because intervals are closed
        assert!(node.overlaps_with(8, 10));
    }

    #[test]
    fn test_overlaps_disjoint() {
        let node = Node::new(2, 8, ());
        assert!(!node.overlaps_with(9, 12));
    }

    #[test]
    fn test_update_max() {
        let mut node = Node::new(1, 5, ());
        node.update_max(Some(10), Some(7));
        assert_eq!(node.max, 10);
    }

    #[test]
    fn test_update_max_no_children() {
        let mut node = Node::new(1, 5, ());
        node.update_max(None, None);
        assert_eq!(node.max, 5);
    }
}
