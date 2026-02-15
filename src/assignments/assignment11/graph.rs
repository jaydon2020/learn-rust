//! A small graph library.
//!
//! A node has a i32 value and (directed) edges to other nodes. A node does not have multiple edges
//! to the same node. Nodes are not associated with a particular domain, and users can freely
//! create nodes however they like. However, after a node is created, it can be added to a
//! `SubGraph`, which form a subgraph of the graph of all nodes. A node can be added to multiple
//! subgraphs. `SubGraph` has a method to check if the it has a cycle.
//!
//! The goal of this assignment is to learn how to deal with inherently shared mutable data in
//! Rust. Design the types and fill in the `todo!()`s in methods. There are several possible
//! approaches to this problem and you may import anything from the std library accordingly.
//!
//! Refer `graph_grade.rs` for test cases.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
enum VisitStatus {
    Unvisited,
    Visiting,
    Visited,
}

/// Handle to a graph node.
// 1. The private inner struct (the actual data in memory)
// We don't need `pub` here because users shouldn't access this directly.
#[derive(Debug, Clone)]
pub struct Node {
    value: i32,
    edges: Vec<NodeHandle>,
}

/// `NodeHandle` should implement `Clone`, which clones the handle without cloning the underlying
/// node. That is, there can be multiple handles to the same node.
/// The user can access the node through a handle if it does not violate Rust's aliasing rules.
///
/// You can freely add fields to this struct.
// 2. The public handle (the lightweight pointer)
#[derive(Debug, Clone)]
pub struct NodeHandle {
    // We wrap our inner Node in Rc (for multiple owners)
    // and RefCell (for interior mutability)
    inner: Rc<RefCell<Node>>,
}

/// Error type for graph operations.
#[derive(Debug)]
pub struct GraphError;

/// Subgraph
///
/// You can freely add fields to this struct.
// 3. The subgraph collection
#[derive(Debug)]
pub struct SubGraph {
    nodes: Vec<NodeHandle>, // The nodes that belong to this subgraph
}

impl NodeHandle {
    /// Creates a node and returns the handle to it.
    pub fn new(value: i32) -> Self {
        let node = Node {
            value,
            edges: Vec::new(),
        };
        NodeHandle {
            inner: Rc::new(RefCell::new(node)),
        }
    }

    /// Adds an edge to `to`.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns
    /// `Err(GraphError)`. Returns `Ok(true)` if the edge is successfully added.
    /// Returns `Ok(false)` if an edge to `to` already exits.
    pub fn add_edge(&self, to: NodeHandle) -> Result<bool, GraphError> {
        // 1. Try to borrow mutably. If it fails, return our custom error.
        let mut node = self.inner.try_borrow_mut().map_err(|_| GraphError)?;

        // 2. Check if the edge already exists.
        // We iterate and check if any edge points to the exact same memory address (ptr_eq)
        let exists = node
            .edges
            .iter()
            .any(|edge| Rc::ptr_eq(&edge.inner, &to.inner));

        if exists {
            Ok(false)
        } else {
            node.edges.push(to);
            Ok(true)
        }
    }

    /// Removes the edge to `to`.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns
    /// `Err(GraphError)`. Returns `Ok(true)` if the edge is successfully removed.
    /// Returns `Ok(false)` if an edge to `to` does not exist.
    pub fn remove_edge(&self, to: &NodeHandle) -> Result<bool, GraphError> {
        // 1. Safe mutable borrow
        let mut node = self.inner.try_borrow_mut().map_err(|_| GraphError)?;

        // 2. Find the INDEX of the node.
        // .position() returns Option<usize> (Some(index) or None)
        let index_opt = node
            .edges
            .iter()
            .position(|edge| Rc::ptr_eq(&edge.inner, &to.inner));

        if let Some(index) = index_opt {
            // Found it! Remove at that index.
            let _unused = node.edges.remove(index);
            // Optimization tip: node.edges.swap_remove(index) is faster
            // if order doesn't matter!
            Ok(true)
        } else {
            // Not found
            Ok(false)
        }
    }

    /// Removes all edges.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns
    /// `Err(GraphError)`.
    pub fn clear_edges(&self) -> Result<(), GraphError> {
        let mut node = self.inner.try_borrow_mut().map_err(|_| GraphError)?;
        node.edges.clear();
        Ok(())
    }
}

impl Default for SubGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SubGraph {
    /// Creates a new subgraph.
    pub fn new() -> Self {
        SubGraph { nodes: Vec::new() }
    }

    /// Adds a node to the subgraph. Returns true iff the node is newly added.
    pub fn add_node(&mut self, node: NodeHandle) -> bool {
        if self
            .nodes
            .iter()
            .any(|edge| Rc::ptr_eq(&edge.inner, &node.inner))
        {
            false
        } else {
            self.nodes.push(node);
            true
        }
    }

    /// Removes a node from the subgraph. Returns true iff the node is successfully removed.
    pub fn remove_node(&mut self, node: &NodeHandle) -> bool {
        let index_opt = self
            .nodes
            .iter()
            .position(|edge| Rc::ptr_eq(&edge.inner, &node.inner));
        if let Some(index) = index_opt {
            let _unused = self.nodes.remove(index);
            true
        } else {
            false
        }
    }

    /// Returns true iff the subgraph contains a cycle. Nodes that do not belong to this subgraph
    /// are ignored. See <https://en.wikipedia.org/wiki/Cycle_(graph_theory)> for an algorithm.
    pub fn detect_cycle(&self) -> bool {
        // We need a way to quickly check if a neighbor is in the subgraph
        // A HashSet of "memory addresses" (pointers) is perfect for this.
        // (We cast the raw pointer to usize to store it in a set)
        let subgraph_nodes: HashSet<usize> = self
            .nodes
            .iter()
            .map(|n| n.inner.as_ptr() as usize)
            .collect();

        // 1. "visiting" (Gray): Nodes currently in the recursion stack
        let mut visiting = HashSet::new();
        // 2. "visited" (Black): Nodes we are completely done with
        let mut visited = HashSet::new();

        // Helper function for DFS
        // Returns true if a cycle is found
        fn dfs(
            current: &NodeHandle,
            visiting: &mut HashSet<usize>,
            visited: &mut HashSet<usize>,
            subgraph_nodes: &HashSet<usize>,
        ) -> bool {
            let id = current.inner.as_ptr() as usize;

            // Hint:
            // 1. If 'id' is in 'visiting', we found a cycle! Return true.
            if visiting.contains(&id) {
                return true;
            }
            // 2. If 'id' is in 'visited', we already checked it. Return false.
            if visited.contains(&id) {
                return false;
            }

            // 3. Mark 'id' as visiting.
            let _ = visiting.insert(id);

            // 4. Get the neighbors. (Don't forget try_borrow!)
            //    If you can't borrow (RefCell error), treat it as no cycle (or panic, but usually ignore).
            if let Ok(node) = current.inner.try_borrow() {
                for neighbor in &node.edges {
                    let neighbor_id = neighbor.inner.as_ptr() as usize;

                    // CRITICAL: Only traverse if the neighbor is part of this subgraph!
                    if subgraph_nodes.contains(&neighbor_id) {
                        // Recurse! If dfs(...) returns true, we return true immediately.
                        if visiting.contains(&neighbor_id) {
                            return true;
                        }
                        if !visited.contains(&neighbor_id)
                            && dfs(neighbor, visiting, visited, subgraph_nodes)
                        {
                            return true;
                        }
                    }
                }
            }

            // 5. We are done with this node.
            //    Remove from 'visiting', add to 'visited'.
            let _ = visiting.remove(&id);
            let _ = visited.insert(id);

            false
        }

        // Outer loop: Launch DFS from every node that hasn't been visited yet
        for node in &self.nodes {
            let id = node.inner.as_ptr() as usize;
            if !visited.contains(&id) && dfs(node, &mut visiting, &mut visited, &subgraph_nodes) {
                return true;
            }
        }

        false
    }
}
