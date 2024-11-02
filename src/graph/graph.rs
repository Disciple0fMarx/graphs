use crate::graph::{Vertex, Edge};
use std::collections::{HashMap, HashSet};

/// A struct representing a graph, which can be either directed or undirected.
/// The graph consists of vertices stored in a `HashMap` and edges connecting them.
pub struct Graph {
    /// A collection of vertices in the graph, where each vertex is indexed by its value.
    pub vertices: HashMap<String, Vertex>,
    /// A boolean indicating if the graph is directed (`true`) or undirected (`false`).
    pub directed: bool,
    /// The number of edges in the graph.
    pub edge_count: usize,
}


impl Graph {
    /// Creates a new `Graph` instance.
    ///
    /// # Arguments
    ///
    /// * `directed` - A `bool` indicating if the graph should be directed (`true`) or undirected (`false`).
    ///
    /// # Returns
    ///
    /// * A `Graph` instance with an empty set of vertices.
    pub fn new(directed: bool) -> Graph {
        Graph {
            vertices: HashMap::new(),
            directed,
            edge_count: 0,
        }
    }
    
    /// Adds a vertex to the graph.
    ///
    /// # Arguments
    ///
    /// * `vertex` - A `Vertex` instance to be added to the graph.
    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.insert(vertex.value.clone(), vertex);
    }

    /// Adds an edge between two vertices in the graph.
    ///
    /// # Arguments
    /// * `edge` - The `Edge` struct connecting two vertices.
    ///
    /// # Returns
    /// * `Result<(), String>` - Returns `Ok(())` if the edge is added successfully; 
    /// returns an error if either vertex is missing.
    ///
    /// # Behavior
    /// * In directed graphs, adds the edge from `vertex1` to `vertex2`.
    /// * In undirected graphs, adds the edge to both vertices to represent an undirected connection.
    ///
    /// # Side Effects
    /// * Increments `edge_count` by one.
    pub fn add_edge(&mut self, edge: Edge) -> Result<(), String> {
        let vertex1_key: String = edge.vertex1.value.clone();
        let vertex2_key: String = edge.vertex2.value.clone();

        let vertices: &mut HashMap<String, Vertex> = &mut self.vertices;

        let vertex1: &mut Vertex = vertices.get_mut(&vertex1_key).ok_or(format!("Vertex {} does not exist", vertex1_key))?;

        vertex1.add_edge(edge.clone());

        if !self.directed {
            let vertex2: &mut Vertex = vertices.get_mut(&vertex2_key).ok_or(format!("Vertex {} does not exist", vertex2_key))?;
            vertex2.add_edge(edge);
        }

        self.edge_count += 1;
        Ok(())
    }

    /// Displays the graph by printing each vertex and its connected edges.
    ///
    /// The output shows each vertex key, followed by a list of vertices it is connected to.
    pub fn display(&self) {
        println!("Graph (Directed: {}):", self.directed);
        for (vertex_key, vertex) in &self.vertices {
            let edges: Vec<String> = vertex.edges.iter()
                .map(|e| format!("({}, {}) (weight: {})", e.vertex1.value, e.vertex2.value, e.weight))
                .collect();
            println!("{}: {:?}", vertex_key, edges);
        }
    }

    /// Creates an adjacency matrix for the graph.
    ///
    /// # Returns
    ///
    /// A 2D `Vec<Vec<Option<u32>>>` representing the adjacency matrix. `None` means no edge exists,
    /// and `Some(weight)` contains the weight of an edge between vertices.
    pub fn adjacency_matrix(&self) -> Vec<Vec<Option<f32>>> {
        let mut index_map: HashMap<&String, usize> = HashMap::new();
        let mut index: usize = 0;
        for vertex in self.vertices.keys() {
            index_map.insert(vertex, index);
            index += 1;
        }

        let size: usize = self.vertices.len();
        let mut matrix: Vec<Vec<Option<f32>>> = vec![vec![None; size]; size];

        for vertex in self.vertices.values() {
            for edge in &vertex.edges {
                let i: usize = *index_map.get(&edge.vertex1.value).unwrap();
                let j: usize = *index_map.get(&edge.vertex2.value).unwrap();
                matrix[i][j] = Some(edge.weight);

                if !self.directed {
                    matrix[j][i] = Some(edge.weight);
                }
            }
        }

        matrix
    }

    /// Displays the graph's adjacency matrix.
    pub fn display_adjacency_matrix(&self) {
        let matrix: Vec<Vec<Option<f32>>> = self.adjacency_matrix();

        println!("Adjacency Matrix:");
        for row in &matrix {
            for &val in row {
                match val {
                    Some(weight) => print!("{:>4}", weight),
                    None => print!("{:>4}", 0),
                }
            }
            println!();
        }
    }

    /// Creates an incidence matrix for the graph.
    ///
    /// # Returns
    ///
    /// A 2D `Vec<Vec<f32>>` representing the incidence matrix.
    /// For undirected graphs, entries contain the edge weight (or `1` if unweighted).
    /// For directed graphs, entries contain `weight` for start vertices and `-weight` for end vertices.
    pub fn incidence_matrix(&self) -> Vec<Vec<f32>> {
        let mut vertex_index: HashMap<&String, usize> = HashMap::new();
        let mut index: usize = 0;
        for vertex in self.vertices.keys() {
            vertex_index.insert(vertex, index);
            index += 1;
        }
    
        let num_vertices: usize = self.vertices.len();
        let mut edges_seen: HashSet<(String, String)> = HashSet::new();
        let mut edges: Vec<(String, String, f32)> = Vec::new();
    
        for vertex in self.vertices.values() {
            for edge in &vertex.edges {
                let (v1, v2) = (edge.vertex1.value.clone(), edge.vertex2.value.clone());
                if !edges_seen.contains(&(v1.clone(), v2.clone())) && !edges_seen.contains(&(v2.clone(), v1.clone())) {
                    edges_seen.insert((v1.clone(), v2.clone()));
                    edges.push((v1, v2, edge.weight));
                }
            }
        }
    
        let num_edges: usize = edges.len();
        let mut matrix: Vec<Vec<f32>> = vec![vec![0.0; num_edges]; num_vertices];
    
        for (edge_index, (v1, v2, weight)) in edges.iter().enumerate() {
            let i: usize = *vertex_index.get(v1).unwrap();
            let j: usize = *vertex_index.get(v2).unwrap();
    
            if self.directed {
                matrix[i][edge_index] = weight.clone();
                matrix[j][edge_index] = -weight;
            } else {
                matrix[i][edge_index] = weight.clone();
                matrix[j][edge_index] = weight.clone();
            }
        }
    
        matrix
    }
    
    /// Displays the graph's incidence matrix.
    pub fn display_incidence_matrix(&self) {
        let matrix: Vec<Vec<f32>> = self.incidence_matrix();
        
        println!("Incidence Matrix:");
        for row in &matrix {
            for &val in row {
                print!("{:>4}", val);
            }
            println!();
        }
    }

    /// Generates the graph's Compressed Sparse Row _(CSR)_ representation with `adjacents` and `indices`.
    pub fn csr_representation(&self) -> (Vec<Option<String>>, Vec<usize>) {
        let edge_count: usize = self.edge_count;
        let mut adjacents: Vec<Option<String>> = Vec::with_capacity(2 * edge_count + 1);
        let mut indices: Vec<usize> = vec![0; self.vertices.len() + 1];
    
        let mut vertex_indices: HashMap<&String, usize> = HashMap::new();
        let mut index: usize = 0;
        for (vertex_key, _) in &self.vertices {
            vertex_indices.insert(vertex_key, index);
            index += 1;
        }
    
        for (vertex_key, vertex) in &self.vertices {
            let vertex_index: usize = vertex_indices[&vertex_key];
    
            for edge in &vertex.edges {
                let neighbor_value: &String = if &edge.vertex1.value == vertex_key {
                    &edge.vertex2.value
                } else {
                    &edge.vertex1.value
                };
    
                adjacents.push(Some(neighbor_value.clone()));
                indices[vertex_index + 1] += 1;
            }
        }
    
        for i in 1..indices.len() {
            indices[i] += indices[i - 1];
        }
    
        adjacents.push(None); 

        (adjacents, indices)
    }

    /// Displays the Compressed Sparse Row _(CSR)_ representation of the graph.
    ///
    /// * Prints `Indices`, showing the starting positions of each vertex's neighbors in `Adjacents`.
    /// * Prints `Adjacents`, displaying each vertex's neighbors as a list, with `None` for empty slots.
    pub fn display_csr_representation(&self) {
        let (adjacents, indices) = self.csr_representation();
    
        println!("Indices: {:?}", indices);
    
        let adjacents_display: String = adjacents.iter()
        .map(|neighbor: &Option<String>| match neighbor {
            Some(value) => value.clone(),
            None => "None".to_string(),  
        })
        .collect::<Vec<String>>()
        .join(", "); // Join with a comma and space

        println!("Adjacents: [{}]", adjacents_display);
    }
}
