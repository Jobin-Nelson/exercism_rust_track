pub mod graph {
    use std::collections::HashMap;
    pub use graph_items::*;

    #[derive(Debug, Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.into();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.into();
            self
        }

        pub fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
            self.attrs = attributes.iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect();
            self
        }

        pub fn node(&self , node: &str) -> Option<&Node> {
            self.nodes.iter()
                .find(|n| n.value == node)
        }

    }

    pub mod graph_items {
        use std::collections::HashMap;

        #[derive(Debug, Clone, PartialEq)]
        pub struct Node {
            pub value: String,
            pub attrs: HashMap<String, String>,
        }
        impl Node {
            pub fn new(value: &str) -> Self {
                Self {
                    value: value.to_owned(),
                    attrs: HashMap::new(),
                }
            }

            pub fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
                self.attrs = attributes.iter()
                    .map(|(a, b)| (a.to_string(), b.to_string()))
                    .collect();
                self
            }

            pub fn attr(&self, attr: &str) -> Option<&str> {
                self.attrs.get(attr).map(|s| s.as_str())
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Edge {
            from: String,
            to: String,
            attrs: HashMap<String, String>,
        }

        impl Edge {
            pub fn new(from: &str, to: &str) -> Self {
                Self {
                    from: from.to_string(),
                    to: to.to_string(),
                    attrs: HashMap::new(),
                }
            }

            pub fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
                self.attrs = attributes.iter()
                    .map(|(a, b)| (a.to_string(), b.to_string()))
                    .collect();
                self
            }

            pub fn attr(&self, val: &str) -> Option<&str> {
                self.attrs.get(val).map(|s| s.as_str())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::graph::*;
    #[test]
    fn empty_graph() {
        let graph = Graph::new();
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
        assert!(graph.attrs.is_empty());
    }

    #[test]
    fn graph_with_one_node() {
        let nodes = vec![Node::new("a")];
        let graph = Graph::new().with_nodes(&nodes);
        assert!(graph.edges.is_empty());
        assert!(graph.attrs.is_empty());
        assert_eq!(graph.nodes, vec![Node::new("a")]);
    }
    #[test]
    fn graph_with_one_node_with_keywords() {
        let nodes = vec![Node::new("a").with_attrs(&[("color", "green")])];
        let graph = Graph::new().with_nodes(&nodes);
        assert!(graph.edges.is_empty());
        assert!(graph.attrs.is_empty());
        assert_eq!(
            graph.nodes,
            vec![Node::new("a").with_attrs(&[("color", "green")])]
        );
    }
    #[test]
    fn graph_with_one_edge() {
        let edges = vec![Edge::new("a", "b")];
        let graph = Graph::new().with_edges(&edges);
        assert!(graph.nodes.is_empty());
        assert!(graph.attrs.is_empty());
        assert_eq!(graph.edges, vec![Edge::new("a", "b")]);
    }
    #[test]
    fn graph_with_one_edge_with_keywords() {
        let edges = vec![Edge::new("a", "b").with_attrs(&[("color", "blue")])];
        let graph = Graph::new().with_edges(&edges);
        assert!(graph.nodes.is_empty());
        assert!(graph.attrs.is_empty());
        assert_eq!(
            graph.edges,
            vec![Edge::new("a", "b").with_attrs(&[("color", "blue")])]
        );
    }
    #[test]
    fn edges_store_attributes() {
        let nodes = vec![
            Node::new("a").with_attrs(&[("color", "green")]),
            Node::new("c"),
            Node::new("b").with_attrs(&[("label", "Beta!")]),
        ];
        let edges = vec![
            Edge::new("b", "c"),
            Edge::new("a", "b").with_attrs(&[("color", "blue"), ("fill", "darkblue")]),
        ];
        let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];
        let graph = Graph::new()
            .with_nodes(&nodes)
            .with_edges(&edges)
            .with_attrs(&attrs);
        assert_eq!(
            graph.edges,
            vec![
                Edge::new("b", "c"),
                Edge::new("a", "b").with_attrs(&[("color", "blue"), ("fill", "darkblue")]),
            ]
        );
        assert_eq!(graph.edges[1].attr("color"), Some("blue"));
        assert_eq!(graph.edges[1].attr("fill"), Some("darkblue"));
        assert_eq!(graph.edges[1].attr("foo"), None);
        assert_eq!(graph.edges[0].attr("color"), None);
        assert_eq!(graph.edges[0].attr("fill"), None);
        assert_eq!(graph.edges[0].attr("foo"), None);
    }
    #[test]
    fn graph_nodes_store_attributes() {
        let attributes = [("foo", "bar"), ("bat", "baz"), ("bim", "bef")];
        let graph = Graph::new().with_nodes(
            &["a", "b", "c"]
                .iter()
                .zip(attributes.iter())
                .map(|(name, &attr)| Node::new(name).with_attrs(&[attr]))
                .collect::<Vec<_>>(),
        );
        let a = graph.node("a").expect("node a must be stored");
        assert_eq!(a.attr("foo"), Some("bar"));
        assert_eq!(a.attr("bat"), None);
        assert_eq!(a.attr("bim"), None);
        let b = graph.node("b").expect("node b must be stored");
        assert_eq!(b.attr("foo"), None);
        assert_eq!(b.attr("bat"), Some("baz"));
        assert_eq!(b.attr("bim"), None);
        let c = graph.node("c").expect("node c must be stored");
        assert_eq!(c.attr("foo"), None);
        assert_eq!(c.attr("bat"), None);
        assert_eq!(c.attr("bim"), Some("bef"));
    }
}
