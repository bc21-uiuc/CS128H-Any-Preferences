use std::collections::HashMap

pub struct Node {
  pub children: Vec<Node>,
  pub node_type: NodeType,
}

enum NodeType { // all node types: https://dom.spec.whatwg.org/#dom-node-nodetype
  Text(String),
  Element(ElementData),
}

pub struct ElementData {
  pub tag_name: String,
  pub attributes: HashMap<String, String>,
}

pub fn text(data: String) -> Node {
  Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

pub fn element(tag_name: String, attributes: HashMap<String, String>, children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: NodeType::Element(ElementData {
      tag_name: tag_name,
      attributes: attributes,
    })
  }
}