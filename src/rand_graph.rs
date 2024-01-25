
use petgraph::matrix_graph::UnMatrix;
use rand::Rng;

pub fn rand_matrix(num_nodes: u32, edge_prob: f64) -> UnMatrix<u8, u8, Option<u8>, u32> {
    let mut rng = rand::thread_rng();
    if edge_prob > 1.0 || edge_prob < 0.0 {
        panic!("Illegal edge probability: {:?}", edge_prob);
    }

    // Create an empty matrix graph
    let mut matrix_graph: UnMatrix<u8, u8, Option<u8>, u32> = UnMatrix::with_capacity(num_nodes as usize);

    // Add nodes to the graph
    for _ in 0..num_nodes {
        matrix_graph.add_node(0);
    }

    // Add random edges to the graph
    for i in 0..num_nodes {
        for j in i + 1..num_nodes {
            if rng.gen_bool(edge_prob) {
                matrix_graph.add_edge(i.into(), j.into(), 1);
            }
        }
    }
    matrix_graph
}