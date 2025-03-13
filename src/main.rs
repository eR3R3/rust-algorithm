use Algorithm::graph::dfs::Graph as dfsGraph;
use Algorithm::graph::bfs::Graph as bfsGraph;
use Algorithm::binary_tree;

fn main() {
    // dfs implementation
    // let mut graph: dfsGraph = dfsGraph::new();
    // graph.add_edge(0, 1);
    // graph.add_edge(1, 2);
    // graph.add_edge(2, 3);
    // graph.add_edge(3, 0);
    // graph.dfs(0);

    // bfs implementation
    // let mut graph: bfsGraph = bfsGraph::new();
    // graph.add_edge(0, 1);
    // graph.add_edge(1, 2);
    // graph.add_edge(2, 3);
    // graph.add_edge(3, 0);
    // graph.add_edge(3,2);
    // graph.add_edge(2,5);
    // graph.add_edge(5,2);
    // graph.add_edge(3,4);
    // graph.bfs(0);

    // binary tree simple implementation
    let mut tree = binary_tree::TreeNode::<i32>::from(5);
    tree.insert(5);
    tree.insert(8);
    let result = tree.search(4);
    println!("result: {}", result)
}
