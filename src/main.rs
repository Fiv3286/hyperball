pub mod plot;
pub mod rand_graph;
pub mod hyperball;

use hyperloglogplus::{HyperLogLog, HyperLogLogPlus};
use petgraph::graph::UnGraph;
use petgraph::{Graph, Undirected};
use petgraph::algo::min_spanning_tree;
use petgraph_gen::random_gnm_graph;


use petgraph::data::FromElements;
use rand::rngs::StdRng;
use rand::RngCore;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::time::Duration;

use crate::hyperball::hyperball;

#[allow(dead_code)]
fn mst_time(g: Graph<(), (), Undirected>) -> (Duration, UnGraph<(), ()>) {
    let start_time = std::time::Instant::now();
    let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&g));
    let elapsed_time = start_time.elapsed();
    (elapsed_time, mst)
}



fn main() {
    let mut rng = rand::thread_rng();

    let v_size: Vec<i64> = (1_000..=1000_000).step_by(1_000).collect();
    let e_size: Vec<i64> = (1_000..=2000_000).step_by(2_000).collect();

    let _points: Vec<()> = v_size
    .iter()
    .zip(e_size.iter())
    .map(|(v, e)| {
        let n = *v as usize;
        let m = *e as usize;
        let g: UnGraph<(), ()> = random_gnm_graph(&mut rng, n, m);

        // let (time, _) = mst_time(g);
        let hllp_counters = hyperball(g);

        let mut merged_counter = hllp_counters
        .into_iter()
        .fold(
            HyperLogLogPlus::new(8, RandomState::new()).unwrap(),
            |acc: HyperLogLogPlus<u8, RandomState>, counter| {
                let mut acc = acc; // Convert acc to a mutable variable
                acc.merge(&counter).unwrap(); // Merge by taking a reference
                acc
            },
        );
        merged_counter.count();



        // let result = (n as u64, time.as_micros() as f64);
        println!("generated hyperball for size {:?} x {:?} without crashing!", n, m);
    })
    .collect();




    // hlpp_vs_hashmap(42, 0x00FF0000);

    

}

#[allow(dead_code)]
fn hlpp_vs_hashmap(seed:u64, bignumber:u32) {
    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(seed);

    let mut hllp: HyperLogLogPlus<u32, _> = HyperLogLogPlus::new(18, RandomState::new()).unwrap();
    let start_time = std::time::Instant::now();
    
    
    // Infinite loop to generate random 64-bit values
    for _ in 0..bignumber {
        // Generate a random 64-bit value
        let random_value: u32 = rng.next_u32();
        hllp.insert(&random_value);
    }
    let elapsed_time = start_time.elapsed();
    println!("hllp; = {:?} in {:?}", hllp.count(), elapsed_time);

    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(42);
    
    let mut hashset = HashSet::new();
    let start_time = std::time::Instant::now();
    for _ in 0..bignumber {
        // Generate a random 64-bit value
        let random_value: u32 = rng.next_u32();
        hashset.insert(random_value);
    }
    let elapsed_time = start_time.elapsed();
    println!("hashset; = {:?} in {:?}", hashset.len(), elapsed_time);
}

    // let rand_adjec = rand_graph::rand_matrix(0x0000FFFF, 0.5f64);
    // let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&rand_adjec));



    
