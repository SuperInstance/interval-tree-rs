//! The interval tree data structure.
//!
//! An augmented BST (binary search tree) where each node stores an interval
//! and the maximum endpoint in its subtree. This enables efficient stabbing
//! and range queries.

use crate::node::Node;

/// An interval tree backed by a BST augmented with max-endpoint values.
///
/// Supports insert, delete, stabbing query (point containment), range query,
/// and overlap detection. All operations work with closed intervals `[low, high]`.
///
/// # Type Parameters
///
/// - `T` — The value type associated with each interval.
///
/// # Examples
///
/// ```
/// use interval_tree_rs::IntervalTree;
///
/// let mut tree: IntervalTree<&str> = IntervalTree::new();
/// tree.insert(1, 5, "a");
/// tree.insert(3, 8, "b");
/// tree.insert(10, 15, "c");
///
/// assert_eq!(tree.len(), 3);
/// assert!(!tree.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct IntervalTree<T> {
    pub(crate) root: Option<Box<InternalNode<T>>>,
    len: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct InternalNode<T> {
    pub(crate) data: Node<T>,
    pub(crate) left: Option<Box<InternalNode<T>>>,
    pub(crate) right: Option<Box<InternalNode<T>>>,
}

impl<T> InternalNode<T> {
    pub(crate) fn max(&self) -> i64 {
        self.data.max
    }

    pub(crate) fn update_max(&mut self) {
        self.data.update_max(
            self.left.as_ref().map(|l| l.max()),
            self.right.as_ref().map(|r| r.max()),
        );
    }
}

impl<T> IntervalTree<T> {
    /// Create a new empty interval tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree_rs::IntervalTree;
    ///
    /// let tree: IntervalTree<i32> = IntervalTree::new();
    /// assert!(tree.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            root: None,
            len: 0,
        }
    }

    /// Returns the number of intervals in the tree.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the tree contains no intervals.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Insert an interval `[low, high]` with an associated value.
    ///
    /// # Panics
    ///
    /// Panics if `low > high`.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree_rs::IntervalTree;
    ///
    /// let mut tree = IntervalTree::new();
    /// tree.insert(1, 10, "first");
    /// tree.insert(5, 15, "second");
    /// assert_eq!(tree.len(), 2);
    /// ```
    pub fn insert(&mut self, low: i64, high: i64, value: T) {
        let node = Node::new(low, high, value);
        self.len += 1;
        self.root = Self::insert_node(self.root.take(), node);
    }

    fn insert_node(current: Option<Box<InternalNode<T>>>, node: Node<T>) -> Option<Box<InternalNode<T>>> {
        match current {
            None => Some(Box::new(InternalNode {
                data: node,
                left: None,
                right: None,
            })),
            Some(mut curr) => {
                if node.low < curr.data.low {
                    curr.left = Self::insert_node(curr.left.take(), node);
                } else {
                    curr.right = Self::insert_node(curr.right.take(), node);
                }
                curr.update_max();
                Some(curr)
            }
        }
    }

    /// Remove an interval with the given `[low, high]` range.
    ///
    /// Returns `Some(value)` if found and removed, `None` otherwise.
    /// If multiple intervals have the same range, removes one of them.
    ///
    /// # Examples
    ///
    /// ```
    /// use interval_tree_rs::IntervalTree;
    ///
    /// let mut tree = IntervalTree::new();
    /// tree.insert(1, 5, "a");
    /// tree.insert(3, 8, "b");
    ///
    /// let removed = tree.delete(1, 5);
    /// assert_eq!(removed, Some("a"));
    /// assert_eq!(tree.len(), 1);
    /// ```
    pub fn delete(&mut self, low: i64, high: i64) -> Option<T> {
        let mut result = None;
        self.root = Self::delete_node(self.root.take(), low, high, &mut result);
        if result.is_some() {
            self.len -= 1;
        }
        result
    }

    fn delete_node(
        current: Option<Box<InternalNode<T>>>,
        low: i64,
        high: i64,
        found: &mut Option<T>,
    ) -> Option<Box<InternalNode<T>>> {
        let mut curr = current?;
        if low < curr.data.low {
            curr.left = Self::delete_node(curr.left.take(), low, high, found);
        } else if low > curr.data.low {
            curr.right = Self::delete_node(curr.right.take(), low, high, found);
        } else if curr.data.high == high {
            // Found the node to delete
            let left = curr.left.take();
            let right = curr.right.take();
            // Destructure to get value — use ManuallyDrop to avoid double-free
            let value = unsafe { std::ptr::read(&curr.data.value) };
            // Don't drop curr through Box — value has been moved out
            std::mem::forget(curr);
            *found = Some(value);

            return match (left, right) {
                (None, None) => None,
                (Some(child), None) => Some(child),
                (None, Some(child)) => Some(child),
                (left, right) => {
                    let (right, mut successor) = Self::extract_min(right.unwrap());
                    successor.left = left;
                    successor.right = right;
                    successor.update_max();
                    Some(successor)
                }
            };
        } else {
            // Same low but different high — search right
            curr.right = Self::delete_node(curr.right.take(), low, high, found);
        }
        curr.update_max();
        Some(curr)
    }

    fn extract_min(mut node: Box<InternalNode<T>>) -> (Option<Box<InternalNode<T>>>, Box<InternalNode<T>>) {
        if node.left.is_none() {
            let right = node.right.take();
            (right, node)
        } else {
            let left = node.left.take().unwrap();
            let (new_left, min_node) = Self::extract_min(left);
            node.left = new_left;
            node.update_max();
            (Some(node), min_node)
        }
    }

    /// Collect all intervals into a vector (in-order traversal).
    ///
    /// Returns references to the stored nodes, sorted by low endpoint.
    pub fn intervals(&self) -> Vec<&Node<T>> {
        let mut result = Vec::new();
        Self::collect_intervals(&self.root, &mut result);
        result
    }

    fn collect_intervals<'a>(node: &'a Option<Box<InternalNode<T>>>, result: &mut Vec<&'a Node<T>>) {
        if let Some(n) = node {
            Self::collect_intervals(&n.left, result);
            result.push(&n.data);
            Self::collect_intervals(&n.right, result);
        }
    }
}

impl<T: Clone> IntervalTree<T> {
    /// Collect all interval values into a vector (in-order traversal).
    pub fn values(&self) -> Vec<T> {
        self.intervals().iter().map(|n| n.value.clone()).collect()
    }
}

impl<T> Default for IntervalTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
