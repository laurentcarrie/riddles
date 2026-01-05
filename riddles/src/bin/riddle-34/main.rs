use std::collections::HashMap;
use std::collections::HashSet;

use crate::riddles::get_product_map;
use crate::riddles::product_of_two_numbers;

pub mod logger;
pub mod riddles;

fn main() {
    crate::logger::init_logger().unwrap();
    log::info!("Starting riddles application");
    let n = 100;

    let possible_numbers_for_1_step_1 = {
        let mut products = get_product_map(n);
        products.retain(|_, v| v.len() > 1);
        products
    };
    log::info!(
        "possible numbers: len={} ; {:?}",
        possible_numbers_for_1_step_1.len(),
        possible_numbers_for_1_step_1
    );

    let impossible_numbers_for_1_step1 = {
        let mut products = get_product_map(n);
        products.retain(|_, v| v.len() == 1);
        products
    };
    log::info!(
        "impossible_numbers_for_1_step1 : len={} ; {:?}",
        impossible_numbers_for_1_step1.len(),
        impossible_numbers_for_1_step1
    );
    // #2 knows that the solution (a,b) is not in impossible_numbers_for_1,
    // this is only possible because a+b is not in map(|a,b| a+b) of impossible_numbers_for_1
    let impossible_numbers_for_2 = {
        let mut ret: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
        for (_product, pairs) in impossible_numbers_for_1_step1.iter() {
            for (a, b) in pairs.iter() {
                let sum = a + b;
                if ret.contains_key(&sum) {
                    ret.get_mut(&sum).unwrap().push((*a, *b));
                } else {
                    ret.insert(sum, vec![(*a, *b)]);
                }
            }
        }
        ret
    };

    // #1 is able go get impossible_numbers_for_2,
    // and from what #2 says, #1 can filter out his solutions
    let possible_numbers_for_1_step2 = {
        let mut ret = possible_numbers_for_1_step_1
            .clone()
            .values()
            .flatten()
            .cloned()
            .collect::<Vec<(u32, u32)>>();
        let impossible_sums_for_2: Vec<u32> = impossible_numbers_for_2.keys().cloned().collect();
        ret.retain(|(a, b)| !impossible_sums_for_2.contains(&(a + b)));
        let mut ret2: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
        for (a, b) in ret.iter() {
            let product = a * b;
            if ret2.contains_key(&product) {
                ret2.get_mut(&product).unwrap().push((*a, *b));
            } else {
                ret2.insert(product, vec![(*a, *b)]);
            }
        }
        ret2
    };

    log::info!(
        "possible_numbers_for_1_step2 : len={} ; {:?}",
        possible_numbers_for_1_step2.len(),
        possible_numbers_for_1_step2
    );

    // #1 knows the solution from product, this means that from previous set we only keep products with a single pair (a,b)
    let possible_numbers_for_1_step3 = {
        let mut ret: HashSet<(u32, u32)> = HashSet::new();
        for (product, pairs) in possible_numbers_for_1_step2.iter() {
            if pairs.len() == 1 {
                ret.insert(pairs[0]);
            }
        }
        ret
    };
    log::info!(
        "possible_numbers_for_1_step3 : len={} ; {:?}",
        possible_numbers_for_1_step3.len(),
        possible_numbers_for_1_step3
    );

    // now #2 knows the solution too
    let possible_numbers_for_2_step3 = {
        let mut ret2: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
        for (a, b) in possible_numbers_for_1_step3.iter() {
            let sum = a + b;
            if impossible_numbers_for_2.contains_key(&sum) {
                continue;
            }
            if ret2.contains_key(&sum) {
                ret2.get_mut(&sum).unwrap().push((*a, *b));
            } else {
                ret2.insert(sum, vec![(*a, *b)]);
            }
        }
        let mut ret: HashSet<(u32, u32)> = HashSet::new();
        for (_sum, pairs) in ret2.iter() {
            if pairs.len() == 1 {
                ret.insert(pairs[0]);
            }
        }
        ret
    };

    log::info!(
        "possible_numbers_for_2_step3 : len={} ; {:?}",
        possible_numbers_for_2_step3.len(),
        possible_numbers_for_2_step3
    );

    assert!(possible_numbers_for_2_step3.len() == 1);

    let solution = possible_numbers_for_2_step3.iter().next().unwrap();
    println!(
        "Solution found: a={} b={} (product={}, sum={})",
        solution.0,
        solution.1,
        solution.0 * solution.1,
        solution.0 + solution.1
    );
}
