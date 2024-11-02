use crate::graph::Edge;

/// A struct representing a vertex in a graph.
/// Each vertex has a `value` of type `f32` and a list of `edges` that connect it to other vertices.
#[derive(Clone)]
pub struct Vertex {
    /// The value of the vertex.
    pub value: String,
    /// The list of edges that connect this vertex to other vertices.
    pub edges: Vec<Edge>,
}


impl Vertex {
    /// Creates a new `Vertex` with the given value.
    /// 
    /// # Arguments
    ///
    /// * `value` - A `String` that represents the value of the vertex.
    ///
    /// # Returns
    ///
    /// * A `Vertex` instance with an empty list of edges.
    pub fn new(value: String) -> Vertex {
        Vertex {
            value,
            edges: Vec::new(),
        }
    }

    /// Adds an edge to the vertex.
    ///
    /// # Arguments
    ///
    /// * `edge` - An `Edge` that connects this vertex to another vertex.
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }
}
