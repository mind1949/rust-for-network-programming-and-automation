use petgraph::dot::{Config, Dot};
use petgraph::Graph;

fn main() {
    let mut network = Graph::<&str, &str>::new();
    let router = network.add_node("router");
    let switch1 = network.add_node("swtich1");
    let switch2 = network.add_node("swtich2");
    let server = network.add_node("server");
    let client = network.add_node("client");

    network.extend_with_edges(&[
        (router, switch1),
        (router, switch2),
        (switch1, server),
        (switch2, client),
    ]);

    println!("{:?}", Dot::with_config(&network, &[Config::EdgeNoLabel]));
}
