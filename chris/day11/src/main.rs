//! Solves day 11 of Advent of Code 2025

use std::{borrow::ToOwned, cell::RefCell, collections::HashMap, fmt, rc::Rc};

/// Represents a node in the graph
struct Node {
    name: String,
    /// Incoming connections
    incoming: Vec<Link>,
    /// Outgoing connections
    outgoing: Vec<Link>,
}

type Link = Rc<RefCell<Node>>;

impl Node {
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
#[derive(Debug)]
struct Graph {
    /// Hash map of all nodes by name
    nodes: Rc<RefCell<HashMap<String, Link>>>,
    /// Source node
    sink: Option<Link>,
    /// Sink node
    source: Option<Link>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            .field(
                "incoming",
                &self
                    .incoming
                    .iter()
                    .map(|node| node.borrow().name.clone())
                    .collect::<Vec<String>>(),
            )
            .field(
                "outgoing",
                &self
                    .outgoing
                    .iter()
                    .map(|node| node.borrow().name.clone())
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}

impl Graph {
    /// Create a new empty graph
    fn new() -> Self {
        Self {
            nodes: Rc::new(RefCell::new(HashMap::new())),
            sink: None,
            source: None,
        }
    }

    /// Get an existing node or add a new empty one and return that if it doesn't exist yet.
    fn get_or_add(&self, name: &str) -> Link {
        let exists = self.nodes.borrow().get(name).is_some();
        if !exists {
            self.add_empty(name);
        }
        self.nodes.borrow().get(name).unwrap().to_owned()
    }

    /// Adds a new empty node to the graph
    #[expect(
        clippy::unwrap_used,
        reason = "Panic cannot occur since we inserted the node right before referencing it."
    )]
    fn add_empty(&self, name: &str) -> Link {
        let new_node = Rc::new(RefCell::new(Node::new(name.to_owned())));
        self.nodes.borrow_mut().insert(name.to_owned(), new_node);
        self.nodes.borrow().get(name).unwrap().to_owned()
    }

    /// Connect two nodes in the graph
    fn connect(parent: Link, child: Link) {
        parent.borrow_mut().outgoing.push(child.clone());
        child.borrow_mut().incoming.push(parent.clone());
    }

    /// Adds a node with outgoing edges, updating both forward and backward refefences
    fn add_node(&self, name: &str, outgoing: Vec<&str>) {
        // NOTE: Due to problems with the borrow checker not allowing us to call
        // get_mut_or_add twice, we need to
        let new_node = self.get_or_add(name);
        for node_str in outgoing {
            let node = self.get_or_add(node_str);
            Graph::connect(new_node.clone(), node);
        }
        ()
        // match self.nodes.get(name) {
        //     Some(existing) =>
        // }
    }
}

#[expect(clippy::print_stdout, reason = "CLI function must report output.")]
fn main() {
    let grph = Graph::new();
    grph.add_empty("Mango");
    grph.add_node("you", vec!["Mango"]);
    println!("Hello, graph!\n{grph:?}");
}
