use anyhow::Context;
use std::str::FromStr;

pub struct NodeSpec {
    pub name: String,
    pub children: Vec<String>,
}

impl FromStr for NodeSpec {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let name = parts.next().context("Missing name")?.to_string();
        let children = parts
            .next()
            .context("Missing children")?
            .split(' ')
            .map(ToString::to_string)
            .collect();

        Ok(NodeSpec { name, children })
    }
}

pub fn parse<'a>(lines: impl IntoIterator<Item = &'a str>) -> anyhow::Result<Vec<NodeSpec>> {
    lines
        .into_iter()
        .enumerate()
        .map(|(line, s)| {
            s.parse::<NodeSpec>()
                .context(format!("Unable to parse node spec (line {})", line + 1))
        })
        .collect()
}
