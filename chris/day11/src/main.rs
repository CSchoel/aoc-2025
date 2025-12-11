//! Solves day 11 of Advent of Code 2025
extern crate alloc;
use alloc::borrow::ToOwned as _;
use alloc::fmt;
use alloc::rc::Rc;
use core::cell::RefCell;
use log::debug;
use std::collections::HashMap;

/// Represents a node in the graph
struct Node {
    /// Incoming connections
    incoming: Vec<Link>,
    /// Name of the node
    name: String,
    /// Outgoing connections
    outgoing: Vec<Link>,
}

/// Link to a node
pub(crate) type Link = Rc<RefCell<Node>>;

impl Node {
    /// Create a new node without any connections
    const fn new(name: String) -> Self {
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

    /// Adds a node with outgoing edges, updating both forward and backward refefences
    fn add_node(&self, name: &str, outgoing: Vec<&str>) {
        // NOTE: Due to problems with the borrow checker not allowing us to call
        // get_mut_or_add twice, we need to
        debug!("Adding node {name:?} with outgoing {outgoing:?}");
        let new_node = self.get_or_add(name);
        for node_str in outgoing {
            let node = self.get_or_add(node_str);
            Self::connect(&new_node, &node);
        }
        // match self.nodes.get(name) {
        //     Some(existing) =>
        // }
    }
    /// Connect two nodes in the graph
    fn connect(parent: &Link, child: &Link) {
        let parent_name = parent.borrow().name.clone();
        let child_name: String = child.borrow().name.clone();
        debug!("Connecting nodes: {parent_name} -> {child_name}");
        parent.borrow_mut().outgoing.push(Rc::clone(child));
        child.borrow_mut().incoming.push(Rc::clone(parent));
    }
    /// Get an existing node or add a new empty one and return that if it doesn't exist yet.
    #[expect(
        clippy::unwrap_used,
        reason = "Panic cannot happen since we check for existance and add before accessing the value."
    )]
    fn get_or_add(&self, name: &str) -> Link {
        let exists = self.nodes.borrow().get(name).is_some();
        if !exists {
            self.add_empty(name);
        }
        self.nodes.borrow().get(name).unwrap().to_owned()
    }

    /// Create a new empty graph
    fn new() -> Self {
        Self {
            nodes: Rc::new(RefCell::new(HashMap::new())),
            sink: None,
            source: None,
        }
    }
}

#[expect(clippy::print_stdout, reason = "CLI function must report output.")]
fn main() {
    env_logger::init();
    let grph = Graph::new();
    grph.add_empty("Mango");
    grph.add_node("you", vec!["Mango"]);
    debug!("Test graph: {grph:?}");
    println!("Result TBD");
}
