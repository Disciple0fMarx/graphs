# Graphs in Rust

This Rust project provides an implementation of graph structures, supporting both directed and undirected graphs. It includes adjacency and incidence matrix representations as well as a compressed sparse row (CSR) format for efficient storage and traversal.

## Features
- **Vertices and Edges**: Add vertices and edges with optional weights.
- **Representations**: View graphs as adjacency matrices, incidence matrices, and CSR.
- **Matrix Display**: Custom methods to display each representation in the console.

## Getting Started
1. **Clone the repository**:
   ```bash
   git clone https://github.com/Disciple0fMarx/graphs.git
   cd graphs
   ```
2. **Build and Run**:
   ```bash
   cargo run
   ``` 

## Usage
To use the library, import it in your Rust code:
```rust
use graph::{Graph, Vertex, Edge};
```

### Creating a Graph
To create a new graph, use the `Graph::new` method:
```rust
let mut graph: Graph = Graph::new(false); // `false` indicates the graph is undirected
```

### Adding Vertices and Edges
To add vertices and edges to the graph, use the `add_vertex` and `add_edge` methods:
```rust
// Create vertices
let vertex1: Vertex = Vertex::new("A".to_string());
let vertex2: Vertex = Vertex::new("B".to_string());
let vertex3: Vertex = Vertex::new("C".to_string());

// Add vertices to the graph
graph.add_vertex(vertex1);
graph.add_vertex(vertex2);
graph.add_vertex(vertex3);

// Create edges between vertices
let edge1: Edge = Edge::new(
    graph.vertices.get("A").unwrap().clone(),
    graph.vertices.get("C").unwrap().clone(),
    1.0,
);

let edge2: Edge = Edge::new(
    graph.vertices.get("C").unwrap().clone(),
    graph.vertices.get("B").unwrap().clone(),
    3.5,
);

// Add the edges to the graph
graph.add_edge(edge1).unwrap_or_else(|e: String| println!("Error: {}", e));
graph.add_edge(edge2).unwrap_or_else(|e: String| println!("Error: {}", e));
```

### Displaying Graph Representations
To display the graph's representations, use the `display` methods:
```rust
// Display the graph
graph.display();
println!();

// Display the adjacency matrix
graph.display_adjacency_matrix();
println!();

// Display the incidence matrix
graph.display_incidence_matrix();
println!();

// Display the CSR representation
graph.display_csr_representation();
```

## Contributing
Contributions are welcome! If you find any bugs or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
