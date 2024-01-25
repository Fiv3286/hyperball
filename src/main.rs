pub mod plot;
pub mod rand_graph;
pub mod hyperball;

use hyperloglogplus::{HyperLogLog, HyperLogLogPlus};
use petgraph::graph::UnGraph;
use petgraph::{Graph, Undirected};
use petgraph::algo::min_spanning_tree;
use petgraph_gen::random_gnm_graph;


use petgraph::data::FromElements;
use plotters::backend::BitMapBackend;
use plotters::drawing::IntoDrawingArea;
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
    let precision:u8 = 8;
    let mut rng = rand::thread_rng();

    let ratios: Vec<f64> = (0..=100).map(|x| f64::from(x) *0.1).collect(); // 0 to 100, / 10 to generate floats for ratio.
    
    for ratio in ratios.iter() {
        println!("{:?}", ratio);
        
        let v_size: Vec<i64> = (1_000..=10_000).step_by(5_000).collect();
    
        let e_size: Vec<i64> = v_size.iter().map(|&v| (v as f64 * ratio).floor() as i64).collect();
    
        // Print results
        for (v, e) in v_size.iter().zip(&e_size) {
            println!("v_size: {}, e_size: {}", v, e);
        
            let _points: Vec<()> = v_size
            .iter()
            .zip(e_size.iter())
            .map(|(v, e)| {
                let n = *v as usize;
                let m = *e as usize;
                let g: UnGraph<(), ()> = random_gnm_graph(&mut rng, n, m);
    
                // let (time, _) = mst_time(g);
                let logbook: Vec<(u8, Vec<HyperLogLogPlus<u8, RandomState>>)> = hyperball(g, precision);

                let path = &format!("images/{}x{}p{}.png", v, e, ratio);
                let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();

                for (t, mut entry) in logbook.clone() {

                    let new_harmonic_mean: (f64, i32) = entry.iter_mut().fold((0.0, 0), |acc: (f64, i32), element: &mut HyperLogLogPlus<u8, RandomState>| {
                        (acc.0 + 1.0 / element.count(), acc.1 + 1)
                    });
                    
                    let harmonic_mean: f64 = if new_harmonic_mean.1 == 0 {
                        0.0 // Handle division by zero if the iterator is empty
                    } else {
                        new_harmonic_mean.1 as f64 / new_harmonic_mean.0
                    };
        

                    println!("dimensions {:?}, {:?}, {:?}. t {:?}, mean {:?}", n, m, ratio, t, harmonic_mean)
                }

                let (_t, hllp_counters) = logbook.last().unwrap();
                    
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
        }
    }
    




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



    
