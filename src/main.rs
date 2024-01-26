pub mod hyperball;
pub mod rand_graph;

use csv::WriterBuilder;
use hyperloglogplus::{HyperLogLog, HyperLogLogPlus};
use petgraph::algo::min_spanning_tree;
use petgraph::graph::UnGraph;
use petgraph::{Graph, Undirected};
use petgraph_gen::random_gnm_graph;

use petgraph::data::FromElements;
use rand::rngs::StdRng;
use rand::RngCore;
use core::panic;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
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
    let precision: u8 = 8;

    let g: UnGraph<(), ()> = random_gnm_graph(&mut rng, 10_000_000, 20_000_000);
    let logbook: Vec<(u8, Vec<HyperLogLogPlus<u8, RandomState>>)> = hyperball(g, precision);

    panic!("stop");
    ///

    let ratios: Vec<f64> = (30..=60).map(|x| f64::from(x) * 0.01).collect(); // 0 to 100, / 100 to generate floats for ratio.

    for ratio in ratios.iter() {
        println!("{:?}", ratio);

        let v_size: Vec<i64> = (1_000..=10_000).step_by(4_000).collect();

        let e_size: Vec<i64> = v_size
            .iter()
            .map(|&v| (v as f64 * ratio).floor() as i64)
            .collect();

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

                    let logbook: Vec<(u8, Vec<HyperLogLogPlus<u8, RandomState>>)> =
                        hyperball(g, precision);

                    

                    let mut data: Vec<Vec<(f64, f64, f64)>> = vec![];

                    //todo: load data array!
                    for (timestep, counters) in logbook.clone() {
                        let transformed_inner: Vec<(f64, f64, f64)> = counters
                            .into_iter()
                            .enumerate()
                            .map(|(index, mut hllp)| {
                                // Extracting values from HyperLogLogPlus
                                let value1 = f64::from(timestep); // Assuming filename should be a float
                                let value2 = index as f64; // Index of HyperLogLogPlus counter
                                let value3 = hllp.count(); // Replace 'count' with the actual field of HyperLogLogPlus
                                
                                (value1, value2, value3)
                            })
                            .collect();
                        
                        data.push(transformed_inner);
                    }
                    
                    let flattened_data: Vec<(f64, f64, f64)> = data.into_iter().flatten().collect();



                    let _result = |data: Vec<(f64, f64, f64)>| -> Result<(), Box<dyn Error>> {
                        // Your dat
                
                        let csv_file_path = &format!("images/{}x{}.csv", v, e);
                
                        // Create a CSV file
                        let file = File::create(csv_file_path)?;
                
                        // Create a CSV writer
                        let mut csv_writer = WriterBuilder::new().from_writer(file);
                
                        // Write the header (optional)
                        csv_writer.write_record(&["t", "node", "counter"])?;
                
                        // Write the data
                        for (x, y, z) in data {
                            csv_writer.write_record(&[x.to_string(), y.to_string(), z.to_string()])?;
                        }
                
                        // Flush the writer to ensure all data is written to the file
                        csv_writer.flush()?;
                
                        println!("Data has been written to {}", csv_file_path);
                
                        Ok(())
                    }(flattened_data);

                    // //inspect data in console 
                    // // Iterate over outer vector
                    // for (i, inner_vec) in data.iter().enumerate() {
                    //     println!("Entry {}: ", i);

                    //     // Iterate over inner vector
                    //     for &(value1, value2, value3) in inner_vec.iter() {
                    //         println!("  ({}, {}, {})", value1, value2, value3);
                    //     }
                    // }

                    // println!("{:?}", flattened_data);
                    // panic!("e");
                    
                

                    // chart.draw_series(SurfaceSeries::xoz(
                    //     flattened_data.iter().map(|&(x, _, _)| x),  // X Series
                    //     flattened_data.iter().map(|&(_, _, z)| z),  // Y Series
                    //     |data, x| data[x.trunc() as i64]  // Functor (calculate Z)
                    // ).style(&BLUE.mix(0.2))).unwrap();
                    
                    // panic!("yeet");

                    // chart.draw_series(
                    //     (0..49)
                    //         .map(|x| std::iter::repeat(x).zip(0..49))
                    //         .flatten()
                    //         .map(|(x,z)| {
                    //             Polygon::new(vec![
                    //                 data[x][z],
                    //                 data[x+1][z],
                    //                 data[x+1][z+1],
                    //                 data[x][z+1],
                    //             ], &BLUE.mix(0.3))
                    //         })
                    // ).unwrap();

                    // for (t, mut entry) in logbook.clone() {
                    //     let new_harmonic_mean: (f64, i32) = entry.iter_mut().fold(
                    //         (0.0, 0),
                    //         |acc: (f64, i32), element: &mut HyperLogLogPlus<u8, RandomState>| {
                    //             (acc.0 + 1.0 / element.count(), acc.1 + 1)
                    //         },
                    //     );

                    //     let harmonic_mean: f64 = if new_harmonic_mean.1 == 0 {
                    //         0.0 // Handle division by zero if the iterator is empty
                    //     } else {
                    //         new_harmonic_mean.1 as f64 / new_harmonic_mean.0
                    //     };

                    //     println!(
                    //         "dimensions {:?}, {:?}, {:?}. t {:?}, mean {:?}",
                    //         n, m, ratio, t, harmonic_mean
                    //     )
                    // }

                    // let (_t, hllp_counters) = logbook.last().unwrap();

                    // let mut merged_counter = hllp_counters.into_iter().fold(
                    //     HyperLogLogPlus::new(8, RandomState::new()).unwrap(),
                    //     |acc: HyperLogLogPlus<u8, RandomState>, counter| {
                    //         let mut acc = acc; // Convert acc to a mutable variable
                    //         acc.merge(&counter).unwrap(); // Merge by taking a reference
                    //         acc
                    //     },
                    // );
                    // merged_counter.count();

                    // // let result = (n as u64, time.as_micros() as f64);
                    // println!(
                    //     "generated hyperball for size {:?} x {:?} without crashing!",
                    //     n, m
                    // );
                })
                .collect();
        }
    }

    // hlpp_vs_hashmap(42, 0x00FF0000);
}

#[allow(dead_code)]
fn hlpp_vs_hashmap(seed: u64, bignumber: u32) {
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
