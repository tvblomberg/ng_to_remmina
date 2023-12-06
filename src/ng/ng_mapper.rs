use crate::ng::models::{Connections, Node, NodeType, RDPConnection};
use std::error::Error;

pub fn map_rdp_connections(
    connections: &Connections,
) -> Result<Vec<RDPConnection>, Box<dyn Error>> {
    let rdp_connections: Vec<RDPConnection> = connections
        .connections
        .iter()
        .filter(|p| p.protocol == "RDP")
        .flat_map(|f| create_rdp_connection(f, &None))
        .collect();

    Ok(rdp_connections)
}

fn create_rdp_connection(f: &Node, group: &Option<String>) -> Vec<RDPConnection> {
    let is_group = f.node_type == NodeType::Container;
    let has_nodes = f.nodes.is_some();

    if is_group && !has_nodes {
        return Vec::new();
    }

    if is_group && has_nodes {
        let node = f.clone();
        let rdp_connections: Vec<RDPConnection> = node
            .nodes
            .unwrap()
            .iter()
            .flat_map(|x| create_rdp_connection(x, &Some(node.name.clone())))
            .collect();

        return rdp_connections;
    }

    vec![RDPConnection {
        name: f.name.clone(),
        description: f.description.clone(),
        group: group.clone(),
        username: f.username.clone(),
        domain: f.domain.clone(),
        hostname: f.hostname.clone(),
    }]
}
