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

impl<'links> Node<'links> {
    /// Creata a new node without any connections
    fn new(name: String) -> Self {
        Self {
            name,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }
    }
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

impl<'graph> Graph<'graph> {
    /// Get an existing node or add a new empty one and return that if it doesn't exist yet.
    #[expect(
        clippy::unwrap_used,
        reason = "Cannot panic since we checked for existence. Need to use the field directly to avoid extra borrow."
    )]
    fn get_mut_or_add(&mut self, name: &str) -> &mut Node<'graph> {
        if self.nodes.contains_key(name) {
            self.nodes.get_mut(name).unwrap()
        } else {
            self.add_empty_mut(name)
        }
    }

    /// Get an existing node or add a new empty one and return that if it doesn't exist yet.
    #[expect(
        clippy::unwrap_used,
        reason = "Cannot panic since we checked for existence. Need to use the field directly to avoid extra borrow."
    )]
    fn get_or_add(&mut self, name: &str) -> &Node<'graph> {
        if self.nodes.contains_key(name) {
            self.nodes.get(name).unwrap()
        } else {
            self.add_empty(name)
        }
    }

    /// Adds a new empty node to the graph
    #[expect(
        clippy::unwrap_used,
        reason = "Panic cannot occur since we inserted the node right before referencing it."
    )]
    fn add_empty_mut(&mut self, name: &str) -> &mut Node<'graph> {
        let new_node = Node::new(name.to_owned());
        self.nodes.insert(name.to_owned(), new_node);
        self.nodes.get_mut(name).unwrap()
    }

    /// Adds a new empty node to the graph
    #[expect(
        clippy::unwrap_used,
        reason = "Panic cannot occur since we inserted the node right before referencing it."
    )]
    fn add_empty(&mut self, name: &str) -> &Node<'graph> {
        let new_node = Node::new(name.to_owned());
        self.nodes.insert(name.to_owned(), new_node);
        self.nodes.get(name).unwrap()
    }

    fn connect(&self, parent: &mut Node<'graph>, child: &'graph mut Node<'graph>) {
        parent.outgoing.push(child);
        child.incoming.push(parent);
    }

    /// Adds a node with outgoing edges, updating both forward and backward refefences
    fn add_node(&mut self, name: &str, outgoing: Vec<&str>) {
        // NOTE: Due to problems with the borrow checker not allowing us to call
        // get_mut_or_add twice, we need to
        let new_node = self
            .nodes
            .get_mut(name)
            .unwrap_or(&mut Node::new(name.to_owned()));
        for node_str in outgoing {
            let node = self
                .nodes
                .get_mut(name)
                .unwrap_or(&mut Node::new(name.to_owned()));
        }
        ()
        // match self.nodes.get(name) {
        //     Some(existing) =>
        // }
    }
}

#[expect(clippy::print_stdout, reason = "CLI function must report output.")]
fn main() {
    println!("Hello, world!");
}
