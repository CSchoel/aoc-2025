//! Solves day 11 of Advent of Code 2025
extern crate alloc;
use alloc::borrow::ToOwned as _;
use alloc::collections::VecDeque;
use alloc::fmt;
use alloc::rc::Rc;
use core::cell::RefCell;
use log::{debug, info};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::process::exit;

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
    sink: Rc<RefCell<Option<Link>>>,
    /// Sink node
    source: Rc<RefCell<Option<Link>>>,
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
        let node_ref = self.nodes.borrow().get(name).unwrap().to_owned();
        match name {
            "you" => {
                info!("Found source node {name}!");
                self.source.replace(Some(Rc::clone(&node_ref)));
            }
            "out" => {
                info!("Found sink node {name}!");
                self.sink.replace(Some(Rc::clone(&node_ref)));
            }
            _ => {}
        }

        node_ref
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
        debug!("Connecting nodes: {parent_name} <-> {child_name}");
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
            sink: Rc::new(RefCell::new(None)),
            source: Rc::new(RefCell::new(None)),
        }
    }
}

// fn debug_graph() {
//     let grph = Graph::new();
//     grph.add_empty("Mango");
//     grph.add_node("you", vec!["Mango"]);
//     debug!("Test graph: {grph:?}");
// }

/// Parse input for day 11
fn parse_input(content: &str) -> Result<Graph, String> {
    let grph = Graph::new();
    for line in content.lines() {
        let Some((name, children)) = line.split_once(':') else {
            return Err(format!("Could not parse line {line}"));
        };
        grph.add_node(
            name.trim(),
            children.split_ascii_whitespace().collect::<Vec<&str>>(),
        );
    }
    Ok(grph)
}

/// Finds all nodes that can potenially reach the sink node
fn find_nodes_reaching_sink(grph: &Graph) -> Result<HashSet<String>, String> {
    let Some(sink) = grph.sink.borrow().clone() else {
        return Err("Sink does not exist!".to_owned());
    };
    let mut todo: VecDeque<Link> = VecDeque::new();
    let mut can_reach_sink: HashSet<String> = HashSet::new();
    todo.push_back(sink);
    while !todo.is_empty() {
        let Some(next) = todo.pop_front() else {
            break;
        };
        can_reach_sink.insert(next.borrow().name.clone());
        for incoming in next.borrow().incoming.clone() {
            todo.push_back(incoming);
        }
    }
    Ok(can_reach_sink)
}

#[expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "CLI function must report output."
)]
fn main() {
    env_logger::init();
    let input_path: &Path = Path::new("input.txt");
    let contents: String = match fs::read_to_string(input_path) {
        Ok(str) => str,
        Err(err) => {
            let input_disp = input_path.display();
            eprintln!("Could not read {input_disp}!\nReason: Err({err})");
            exit(1);
        }
    };
    let Ok(input) = parse_input(&contents) else {
        eprintln!("Could not parse input {contents}");
        exit(1);
    };
    info!("Parsed input: {input:?}");
    let can_reach = find_nodes_reaching_sink(&input);
    info!("Nodes that can reach the sink: {can_reach:?}");
    println!("Result TBD");
}
