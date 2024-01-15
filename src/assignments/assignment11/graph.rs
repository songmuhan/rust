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

use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet},
    hash::Hash,
    rc::Rc,
};

#[derive(PartialEq, Eq, Debug)]
enum VisitStatus {
    Unvisited,
    Visiting,
    Visited,
}
/// node
#[derive(Debug, Clone)]
pub struct Node {
    id: i32,
    adjacent: HashSet<i32>,
}
/// new
impl Node {
    /// docs
    pub fn new(value: i32) -> Self {
        Node {
            id: value,
            adjacent: HashSet::new(),
        }
    }
}

/// Handle to a graph node.
/// `NodeHandle` should implement `Clone`, which clones the handle without cloning the underlying
/// node. That is, there can be multiple handles to the same node.
/// The user can access the node through a handle if it does not violate Rust's aliasing rules.
///
/// TODO: You can freely add fields to this struct.
#[derive(Debug, Clone)]
pub struct NodeHandle {
    node: Rc<RefCell<Node>>,
}

/// Error type for graph operations.
#[derive(Debug)]
pub struct GraphError;

/// Subgraph
///
/// TODO: You can freely add fields to this struct.
#[derive(Debug)]
pub struct SubGraph {
    ///
    pub handles: HashMap<i32, NodeHandle>,
    node_set: HashSet<i32>,
}

impl NodeHandle {
    /// Creates a node and returns the handle to it.
    pub fn new(value: i32) -> Self {
        NodeHandle {
            node: Rc::new(RefCell::new(Node::new(value))),
        }
    }

    /// Adds an edge to `to`.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns `Err(GraphError)`.
    /// Returns `Ok(true)` if the edge is successfully added.
    /// Returns `Ok(false)` if an edge to `to` already exits.
    pub fn add_edge(&self, to: NodeHandle) -> Result<bool, GraphError> {
        let to_id = (*to.node).borrow().id;
        if self.node.borrow_mut().adjacent.contains(&to_id) {
            Ok(false)
        } else {
            _ = self.node.borrow_mut().adjacent.insert(to_id);
            Ok(true)
        }
    }

    /// Removes the edge to `to`.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns `Err(GraphError)`.
    /// Returns `Ok(true)` if the edge is successfully removed.
    /// Returns `Ok(false)` if an edge to `to` does not exist.
    pub fn remove_edge(&self, to: &NodeHandle) -> Result<bool, GraphError> {
        let to_id = (*to.node).borrow().id;
        let mut adjacent = &mut self.node.borrow_mut().adjacent;
        if adjacent.contains(&to_id) {
            _ = (*adjacent).remove(&to_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Removes all edges.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns `Err(GraphError)`.
    pub fn clear_edges(&self) -> Result<(), GraphError> {
        self.node.borrow_mut().adjacent.clear();
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
        SubGraph {
            handles: HashMap::new(),
            node_set: HashSet::new(),
        }
    }

    /// Adds a node to the subgraph. Returns true iff the node is newly added.
    pub fn add_node(&mut self, node: NodeHandle) -> bool {
        let id = (*node.node).borrow().id;
        if let std::collections::hash_map::Entry::Vacant(e) = self.handles.entry(id) {
            _ = e.insert(node);
            _ = self.node_set.insert(id);
            true
        } else {
            false
        }
    }

    /// Adds a node to the subgraph. Returns true iff the node is successfully removed.
    pub fn remove_node(&mut self, node: &NodeHandle) -> bool {
        let id = (*node.node).borrow().id;
        if self.handles.contains_key(&id) {
            _ = self.handles.remove(&id);
            _ = self.node_set.remove(&id);
            true
        } else {
            false
        }
    }

    fn has_cycle(&self, node_id: &i32, status: &mut HashMap<i32, VisitStatus>) -> bool {
        println!("{:?}, and checking {:?}", status, node_id);
        if let Some(visit_status) = status.get(node_id) {
            match visit_status {
                VisitStatus::Visiting => return true, // Cycle found
                VisitStatus::Visited => return false, // Already visited, no cycle here
                _ => (),
            }
        }

        // Mark the current node as Visiting
        _ = status.insert(*node_id, VisitStatus::Visiting);

        // Recursively visit all adjacent nodes
        if let Some(node_handle) = self.handles.get(node_id) {
            for &adj_node_id in (*node_handle.node).borrow().adjacent.iter() {
                if self.handles.contains_key(&adj_node_id) && self.has_cycle(&adj_node_id, status) {
                    return true; // Cycle found in a subsequent node
                }
            }
        }
        // Mark the node as Visited after exploring all adjacent nodes
        _ = status.insert(*node_id, VisitStatus::Visited);
        false
    }

    /// Returns true iff the subgraph contains a cycle. Nodes that do not belong to this subgraph
    /// are ignored. See <https://en.wikipedia.org/wiki/Cycle_(graph_theory)> for an algorithm.
    pub fn detect_cycle(&self) -> bool {
        let mut status = HashMap::new();

        // Initialize all nodes as Unvisited
        for &node_id in self.handles.keys() {
            _ = status.insert(node_id, VisitStatus::Unvisited);
        }

        // Check each node for a cycle
        for &node_id in self.handles.keys() {
            if status[&node_id] == VisitStatus::Unvisited && self.has_cycle(&node_id, &mut status) {
                return true; // Cycle found
            }
        }

        false
    }
}
