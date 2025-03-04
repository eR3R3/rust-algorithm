use Algorithm::graph::dfs::Graph;

fn main() {
    // dfs implementation
    let mut graph: Graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(3, 0);
    graph.dfs(0);


}
