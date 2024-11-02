use crate::graph::vertex::Vertex;

/// A struct representing an edge in a graph.
/// An edge connects two vertices and can have an associated weight.
#[derive(Clone)]
pub struct Edge {
    /// The first vertex that the edge connects.
    pub vertex1: Vertex,
    /// The second vertex that the edge connects.
    pub vertex2: Vertex,
    /// The weight of the edge, representing the cost or distance between the two vertices.
    pub weight: f32,
}

// impl PartialEq for Edge {
//     fn eq(&self, other: &Self) -> bool {
//         self.vertex1 == other.vertex1
//             && self.vertex2 == other.vertex2
//             && (self.weight - other.weight).abs() < f32::EPSILON
//     }
// }

impl Edge {
    /// Creates a new `Edge` with the given vertices and weight.
    ///
    /// # Arguments
    ///
    /// * `vertex1` - The first `Vertex` connected by this edge.
    /// * `vertex2` - The second `Vertex` connected by this edge.
    /// * `weight` - A `f32` value representing the weight of the edge.
    ///
    /// # Returns
    ///
    /// * An `Edge` instance connecting the two vertices with the specified weight.
    pub fn new(vertex1: Vertex, vertex2: Vertex, weight: f32) -> Edge {
        Edge {
            vertex1,
            vertex2,
            weight,
        }
    }
}
