use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::algo::{dijkstra, min_spanning_tree};

use petgraph::data::FromElements;
use petgraph::dot::{Dot, Config};

use petgraph::{Graph, Undirected};
use petgraph_gen::random_gnm_graph;

use plotters::data::float;
use plotters::prelude::*;

use std::time::{Duration, Instant};

fn main() {

    //generate random graph.

    let mut rng = rand::thread_rng();
    let n = 1_000_000_0;
    let m = 2_000_000_0;
    let g: UnGraph<(), ()> = random_gnm_graph(&mut rng, n, m);

    
    let v_size: Vec<i64> = (1_000_000..=10_000_000).step_by(1_000_000).collect();
    let e_size: Vec<i64> = (2_000_000..=20_000_000).step_by(2_000_000).collect();

    let g: UnGraph<(), ()> = random_gnm_graph(&mut rng, n, m);

    
    // Get the minimum spanning tree of the graph as a new graph
    let (time, mst) = mst_time(g);
    println!("mst found in {:?}", time);

    



    
    // Find the shortest path from `1` to `4` using `1` as the cost for every edge.
    let node_map = dijkstra(&g, 1.into(), Some(4.into()), |_| 1);
    // Output the tree to `graphviz` `DOT` format
    // println!("{:?}", Dot::with_config(&mst, &[Config::EdgeNoLabel]));
    // // graph {
    // //     0 [label="\"0\""]
    // //     1 [label="\"0\""]
    // //     2 [label="\"0\""]
    // //     3 [label="\"0\""]
    // //     1 -- 2
    // //     3 -- 4
    // //     2 -- 3
    // // }


//drawing area

    let root_drawing_area = BitMapBackend::new("images/0.1.png", (1024, 768))
        .into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .build_cartesian_2d(-3.14..3.14, -1.2..1.2)
        .unwrap();

    chart.draw_series(LineSeries::new(
        (-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
        &RED
    )).unwrap();




}

fn mst_time(graph : Graph<(), (), Undirected>) -> (Duration, Graph<(), (), Undirected>) {
    let start = Instant::now();
    let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&graph));
    let duration = start.elapsed();
    (duration, mst)
}

