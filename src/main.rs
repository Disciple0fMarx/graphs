mod graph;

use graph::{Graph, Vertex, Edge};

fn main() {
    // Create a new graph (directed or undirected)
    let mut graph: Graph = Graph::new(false); // `false` indicates the graph is undirected

    // Create vertices
    let vertex1: Vertex = Vertex::new("A".to_string());
    let vertex2: Vertex = Vertex::new("B".to_string());
    let vertex3: Vertex = Vertex::new("C".to_string());
    let vertex4: Vertex = Vertex::new("D".to_string());
    let vertex5: Vertex = Vertex::new("E".to_string());

    // Add vertices to the graph
    graph.add_vertex(vertex1);
    graph.add_vertex(vertex2);
    graph.add_vertex(vertex3);
    graph.add_vertex(vertex4);
    graph.add_vertex(vertex5);

    // Create edges between vertices
    let edge1: Edge = Edge::new(
        graph.vertices.get("A").unwrap().clone(),
        graph.vertices.get("C").unwrap().clone(),
        1.0,
    );

    let edge2: Edge = Edge::new(
        graph.vertices.get("C").unwrap().clone(),
        graph.vertices.get("E").unwrap().clone(),
        1.0,
    );

    let edge3: Edge = Edge::new(
        graph.vertices.get("E").unwrap().clone(),
        graph.vertices.get("B").unwrap().clone(),
        1.0,
    );

    let edge4: Edge = Edge::new(
        graph.vertices.get("B").unwrap().clone(),
        graph.vertices.get("D").unwrap().clone(),
        1.0,
    );

    let edge5: Edge = Edge::new(
        graph.vertices.get("D").unwrap().clone(),
        graph.vertices.get("A").unwrap().clone(),
        1.0,
    );

    // Add the edges to the graph
    graph.add_edge(edge1).unwrap_or_else(|e: String| println!("Error: {}", e));
    graph.add_edge(edge2).unwrap_or_else(|e: String| println!("Error: {}", e));
    graph.add_edge(edge3).unwrap_or_else(|e: String| println!("Error: {}", e));
    graph.add_edge(edge4).unwrap_or_else(|e: String| println!("Error: {}", e));
    graph.add_edge(edge5).unwrap_or_else(|e: String| println!("Error: {}", e));

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
}
