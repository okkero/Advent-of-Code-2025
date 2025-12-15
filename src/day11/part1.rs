use super::input::{self, NodeSpec};
use anyhow::{Context, Result};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Default)]
struct Node {
    inner: RefCell<NodeInner>,
}

#[derive(Debug, Default)]
struct NodeInner {
    is_out: bool,
    paths_to_out: Option<u32>,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn out() -> Self {
        Self {
            inner: RefCell::new(NodeInner {
                is_out: true,
                ..NodeInner::default()
            }),
        }
    }

    fn paths_to_out(&self) -> u32 {
        if self.inner.borrow().is_out {
            return 1;
        }

        if let Some(paths_to_out) = self.inner.borrow().paths_to_out {
            paths_to_out
        } else {
            let mut inner_mut = self.inner.borrow_mut();
            let children_paths_to_out = inner_mut
                .children
                .iter()
                .map(|node| node.paths_to_out())
                .sum();
            inner_mut.paths_to_out = Some(children_paths_to_out);

            children_paths_to_out
        }
    }
}

fn build_tree(node_specs: Vec<NodeSpec>) -> Result<Rc<Node>> {
    let mut node_map = node_specs
        .into_iter()
        .map(|node_spec| {
            (
                node_spec.name,
                (node_spec.children, Rc::new(Node::default())),
            )
        })
        .collect::<HashMap<_, _>>();

    node_map.insert("out".to_string(), (vec![], Rc::new(Node::out())));

    for (child_names, node) in node_map.values() {
        node.inner.borrow_mut().children.extend(
            child_names
                .iter()
                .map(|child_name| Rc::clone(&node_map[child_name].1)),
        );
    }

    Ok(node_map.remove("you").context("No you in input")?.1)
}

pub fn solve(input: &str) -> Result<()> {
    let parsed = input::parse(input.lines())?;
    let you = build_tree(parsed).context("Error building tree")?;
    let paths_to_out = you.paths_to_out();

    println!("Paths to out: {paths_to_out}");

    Ok(())
}
