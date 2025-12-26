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
    /// Sink node
    sink: Rc<RefCell<Option<Link>>>,
    /// Source node
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

    /// Adds a node with outgoing edges, updating both forward and backward references
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
        reason = "Panic cannot happen since we check for existence and add before accessing the value."
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

/// Finds all nodes that can potentially reach the sink node
#[expect(
    unused,
    reason = "This was just a test function to play with the Graph structure."
)]
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

/// Finds all paths from source to sink
#[expect(
    unused,
    reason = "This is the solution for part 1, which we keep for reference."
)]
fn count_paths_from_source_to_sink(grph: &Graph) -> Result<u32, String> {
    let Some(source) = grph.source.borrow().clone() else {
        return Err("Cannot find source!".to_owned());
    };
    let Some(sink) = grph.sink.borrow().clone() else {
        return Err("Cannot find source!".to_owned());
    };
    count_paths_to_sink(
        &source.borrow().name,
        &Vec::new(),
        &sink.borrow().name,
        grph,
    )
}

/// Find all paths from the `start_node` to the sink, considering only paths that contain all nodes in `must_visit`.
fn count_paths_to_sink(
    start_node: &str,
    must_visit: &Vec<&str>,
    end_node: &str,
    grph: &Graph,
) -> Result<u32, String> {
    let Some(start_name) = grph
        .nodes
        .borrow()
        .get(start_node)
        .map(|x| x.borrow().name.clone())
    else {
        return Err("Source does not exist!".to_owned());
    };
    // (name of node, path to node)
    let mut to_explore: VecDeque<(String, String)> = VecDeque::new();
    let mut explored_paths: HashSet<String> = HashSet::new();
    to_explore.push_back((start_name.clone(), start_name));
    let mut paths_to_sink: HashSet<String> = HashSet::new();
    let nodes = grph.nodes.borrow();
    while !to_explore.is_empty() {
        let Some((next_name, next_path)) = to_explore.pop_front() else {
            continue;
        };
        debug!("Exploring path {next_path}");
        if next_name == end_node {
            info!("Found new path to sink: {next_path}");
            paths_to_sink.insert(next_path.clone());
        }
        if explored_paths.contains(&next_path) {
            debug! {"Path already explored, skipping!"}
            // don't visit the same path twice
            continue;
        }
        explored_paths.insert(next_path.clone());
        let Some(node) = nodes.get(&next_name) else {
            continue;
        };
        for out_node in &node.borrow().outgoing {
            let out_name = out_node.borrow().name.clone();
            to_explore.push_front((out_name.clone(), format!("{next_path}->{out_name}")));
        }
    }
    u32::try_from(
        paths_to_sink
            .iter()
            .filter(|path| must_visit.iter().all(|to_visit| path.contains(to_visit)))
            .count(),
    )
    .map_err(|err| format!("Casting error: {err}"))
}

// /// Find all paths from the `start_node` to the sink, considering only paths that contain all nodes in `must_visit`.
// fn list_paths_from_a_to_b(node_a: &str, node_b: &str)

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
    // let can_reach = find_nodes_reaching_sink(&input);
    // info!("Nodes that can reach the sink: {can_reach:?}");
    let count = match count_paths_to_sink("svr", &vec![], "dac", &input) {
        Ok(cnt) => cnt,
        Err(err) => {
            eprintln!("Could not find count. Reason:\n{err}");
            exit(1);
        }
    };
    println!("Result: {count}");
}
