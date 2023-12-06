use serde::Deserialize;

#[derive(Deserialize)]
pub struct Connections {
    #[serde(rename = "Node")]
    pub connections: Vec<Node>,
}

#[derive(Deserialize, Clone)]
pub struct Node {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Descr")]
    pub description: Option<String>,
    #[serde(rename = "Type")]
    pub node_type: NodeType,
    #[serde(rename = "Username")]
    pub username: Option<String>,
    #[serde(rename = "Domain")]
    pub domain: String,
    #[serde(rename = "Hostname")]
    pub hostname: String,
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "Node")]
    pub nodes: Option<Vec<Node>>,
}

#[derive(Debug)]
pub struct RDPConnection {
    pub name: String,
    pub description: Option<String>,
    pub group: Option<String>,
    pub username: Option<String>,
    pub domain: String,
    pub hostname: String,
}

#[derive(Deserialize, Clone, PartialEq, Eq)]
pub enum NodeType {
    Container,
    Connection,
}
