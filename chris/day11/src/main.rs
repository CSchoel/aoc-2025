//! Solves day 11 of Advent of Code 2025

use std::collections::HashMap;

/// Represents a node in the graph
struct Node<'links> {
    name: String,
    /// Incoming connections
    incoming: Vec<&'links Self>,
    /// Outgoing connections
    outgoing: Vec<&'links Self>,
}

/// Represents a Graph
struct Graph<'links> {
    /// Hash map of all nodes by name
    nodes: HashMap<String, Node<'links>>,
    /// Source node
    sink: Node<'links>,
    /// Sink node
    source: Node<'links>,
}

#[expect(
    clippy::unwrap_used,
    reason = "Cannot panic since we checked for existence. Need to use the field directly to avoid extra borrow."
)]
impl<'graph> Graph<'graph> {
    /// Get an existing node or add a new empty one and return that if it doesn't exist yet.
    fn get_mut_or_add(&mut self, name: &str) -> &mut Node<'graph> {
        if self.nodes.contains_key(name) {
            self.nodes.get_mut(name).unwrap()
        } else {
            self.add_empty(name)
        }
    }

    /// Adds a new empty node to the graph
    #[expect(
        clippy::unwrap_used,
        reason = "Panic cannot occur since we inserted the node right before referencing it."
    )]
    fn add_empty(&mut self, name: &str) -> &mut Node<'graph> {
        let new_node = Node {
            name: name.to_owned(),
            incoming: Vec::new(),
            outgoing: Vec::new(),
        };
        self.nodes.insert(name.to_owned(), new_node);
        self.nodes.get_mut(name).unwrap()
    }

    // /// Adds a node with outgoing edges, updating both forward and backward refefences
    // fn add_node(&mut self, name: &str, outgoing: Vec<&str>) {
    //     let new_node = self.get_mut_or_add(name);
    //     for node_str in outgoing {
    //         let node: &mut Node<'graph> = self.get_mut_or_add(node_str);
    //         node.outgoing.push(new_node);
    //     }
    //     ()
    //     // match self.nodes.get(name) {
    //     //     Some(existing) =>
    //     // }
    // }
}

#[expect(clippy::print_stdout, reason = "CLI function must report output.")]
fn main() {
    println!("Hello, world!");
}
