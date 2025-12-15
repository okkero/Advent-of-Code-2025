use super::input::{self, NodeSpec};
use anyhow::{Context, Result};
use std::{cell::RefCell, collections::HashMap, iter::Sum, ops::Add, rc::Rc};

#[derive(Debug, Default)]
struct Node {
    inner: RefCell<NodeInner>,
}

#[derive(Debug, Default)]
struct NodeInner {
    kind: NodeKind,
    path_trace: Option<PathTrace>,
    children: Vec<Rc<Node>>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
enum NodeKind {
    Dac,
    Fft,
    Out,
    #[default]
    Generic,
}

impl Node {
    fn new(kind: NodeKind) -> Self {
        Self {
            inner: RefCell::new(NodeInner {
                kind,
                ..NodeInner::default()
            }),
        }
    }

    fn trace_path(&self) -> PathTrace {
        let inner = self.inner.borrow();
        let kind = inner.kind;
        if kind == NodeKind::Out {
            return PathTrace {
                total: 1,
                dac: 0,
                fft: 0,
                both: 0,
            };
        }

        drop(inner);

        if let Some(path_trace) = self.inner.borrow().path_trace {
            path_trace
        } else {
            let mut inner_mut = self.inner.borrow_mut();

            let my_trace: PathTrace = inner_mut
                .children
                .iter()
                .map(|child| {
                    let mut child_trace = child.trace_path();

                    if kind == NodeKind::Dac {
                        child_trace.dac = child_trace.total;
                        child_trace.both = child_trace.fft;
                    } else if kind == NodeKind::Fft {
                        child_trace.fft = child_trace.total;
                        child_trace.both = child_trace.dac;
                    }

                    child_trace
                })
                .sum();

            inner_mut.path_trace = Some(my_trace);

            my_trace
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct PathTrace {
    total: u64,
    dac: u64,
    fft: u64,
    both: u64,
}

impl Add for PathTrace {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            total: self.total + rhs.total,
            dac: self.dac + rhs.dac,
            fft: self.fft + rhs.fft,
            both: self.both + rhs.both,
        }
    }
}

impl Sum for PathTrace {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(Add::add).unwrap_or_default()
    }
}

fn build_tree(node_specs: Vec<NodeSpec>) -> Result<Rc<Node>> {
    let mut node_map = node_specs
        .into_iter()
        .map(|node_spec| {
            let node_kind = match node_spec.name.as_str() {
                "dac" => NodeKind::Dac,
                "fft" => NodeKind::Fft,
                _ => NodeKind::Generic,
            };

            (
                node_spec.name,
                (node_spec.children, Rc::new(Node::new(node_kind))),
            )
        })
        .collect::<HashMap<_, _>>();

    node_map.insert(
        "out".to_string(),
        (vec![], Rc::new(Node::new(NodeKind::Out))),
    );

    for (child_names, node) in node_map.values() {
        node.inner.borrow_mut().children.extend(
            child_names
                .iter()
                .map(|child_name| Rc::clone(&node_map[child_name].1)),
        );
    }

    Ok(node_map.remove("svr").context("No svr in input")?.1)
}

pub fn solve(input: &str) -> Result<()> {
    let parsed = input::parse(input.lines())?;
    let svr = build_tree(parsed)?;
    let path_trace = svr.trace_path();

    println!("Path trace: {path_trace:#?}");

    Ok(())
}
