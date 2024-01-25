use std::collections::hash_map::RandomState;

use hyperloglogplus::{HyperLogLog, HyperLogLogPlus};
use petgraph::{graph::UnGraph, visit::IntoNodeIdentifiers};

pub fn regularball(graph: UnGraph<(), (), u32>) -> Vec<i64> { 
    let mut int_counters = Vec::with_capacity(graph.node_count());
    for _i in 0..graph.node_count() {
        int_counters.push(0i64);
    }
    int_counters
}


pub fn hyperball(graph: UnGraph<(), (), u32>, precision:u8) -> Vec<(u8, Vec<HyperLogLogPlus<u8, RandomState>>)> {

    // ; initialize array c with n HyperLogLogs 
    let mut hllp_counters:Vec<HyperLogLogPlus<u8, RandomState>> = Vec::with_capacity(graph.node_count()); // will panic if graph too large n that's okay

    for _i in 0..graph.node_count() {
        hllp_counters.push(HyperLogLogPlus::new(precision, RandomState::new()).unwrap());
    }
    // println!("{:?}", hllp_counters.len());

    for i in graph.node_identifiers() {
        let index = i.index();
        hllp_counters[index].insert(&(index as u8));
    }

    // initialize logbook ; since capacity 0 will re-allocate constantly.... good thing itls only accessed once every t++1 iteration. 
    let mut logbook:Vec<(u8, Vec<HyperLogLogPlus<u8, RandomState>>)> = Vec::with_capacity(0);

    // this block needs to iterate until there's no change anymore. 
    let mut t = 1;
    let mut new_sum:f64 =0.0; 
    let mut old_sum:f64 =1.0;
    
    while old_sum != new_sum {
            old_sum = hllp_counters.iter_mut().fold(None, |acc, element| {
                match acc {
                    None => Some(element.count()),
                    Some(merged_value) => Some(merged_value + element.count()),
                }
            }).unwrap();

            // now iterate 

            for i in graph.node_identifiers() {
                let index = i.index();
                // 
                // let oldcount = hllp_counters[index].count();
                let mut accumulator = hllp_counters[index].clone(); // probably expensive
                for neighbor in graph.neighbors(i) {
                    let nindex = neighbor.index();
                    accumulator.merge(&hllp_counters[nindex]).unwrap();
                    // let (first, rest) = hllp_counters.split_at_mut(index);
                    // let (value1, value2) = first.split_at_mut(index);
                    // value1[0].merge(&value2[0]);
                    
                }
                // mutate main counter to be updated value
                // println!("update node {:?} counter {:?} -> {:?}", i, oldcount, accumulator.count());
                hllp_counters[index] = accumulator;
            };

            //update checksum. 

            logbook.push((t, hllp_counters.clone()));

            t+=1;
            
            new_sum = hllp_counters.iter_mut().fold(None, |acc, element| {
                match acc {
                    None => Some(element.count()),
                    Some(merged_value) => Some(merged_value + element.count()),
                }
            }).unwrap();
            
            // value we're interested in was supposed to be harmonic mean? 
            let new_harmonic_mean: (f64, i32) = hllp_counters.iter_mut().fold((0.0, 0), |acc: (f64, i32), element: &mut HyperLogLogPlus<u8, RandomState>| {
                (acc.0 + 1.0 / element.count(), acc.1 + 1)
            });
            
            let harmonic_mean: f64 = if new_harmonic_mean.1 == 0 {
                0.0 // Handle division by zero if the iterator is empty
            } else {
                new_harmonic_mean.1 as f64 / new_harmonic_mean.0
            };




            // println!("t={:?} oldsum {:?} newsum {:?} mean {:?}", t, old_sum, new_sum, harmonic_mean);
        
            // iteration 1 now complete; check if changed 
    }





    // for each vertex (in neighborhood?)

    // println!("wooo we actually terminated?");
    logbook
// function to combine counter M and N b pairwise comparison of elements; library "merge" method 

//  add(c[v], v) -> get counter @ v, add it's own vertex into it 

// t is 0 

// looop ; 
//     for each v in n 
//      a 


}

// based on [[1308.2144.pdf#page=4]]

// initialize c \[ ] = |G| hyperloglog counters

// initialize all values of c by adding "their" node into it. 

// while no more mutations:  -> how exactly to do this quickly?...

// for each v in n : 
// 	a is counter @ v value
// 	for each (outgoing links w -> neighbor 
// 		a is union ( counter w  and a )
// 	 end 
// 	write (v,a) to disk ?
// 	.... do something with a and c(v) 
// read the pairs v a and update the array


// assumption:
// each register in hll has size ceiling(log log n) 
