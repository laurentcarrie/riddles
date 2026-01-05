use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct CurrentGuess {
    pub data: Vec<(u32, u32)>,
}

pub fn initial(n: u32) -> CurrentGuess {
    let mut ret: Vec<(u32, u32)> = Vec::new();
    for i in 2..n {
        for j in i..n {
            ret.push((i, j));
        }
    }
    CurrentGuess { data: ret }
}

pub fn remove_product_single_pairs(cg: &CurrentGuess) -> CurrentGuess {
    let mut cg = cg.clone();
    let mut product_map: HashMap<u32, usize> = HashMap::new();
    for (a, b) in cg.data.iter() {
        let product = a * b;
        *product_map.entry(product).or_default() += 1;
    }
    cg.data
        .retain(|(a, b)| product_map.get(&(a * b)).expect("logic error") > &1);
    cg
}

pub fn complement_guess(cg: &CurrentGuess, n: u32) -> CurrentGuess {
    let mut ret = initial(n);
    for p in cg.data.iter() {
        log::debug!("complement_guess: remaining pair: {p:?}");
        ret.data.retain(|x| x != p);
    }
    ret
}

pub fn guess_sums(cg: CurrentGuess) -> HashSet<u32> {
    let mut sums: HashSet<u32> = HashSet::new();
    for (a, b) in cg.data.iter() {
        sums.insert(a + b);
    }
    sums
}

pub fn remove_matching_sums(cg: &CurrentGuess, impossible_sums: &HashSet<u32>) -> CurrentGuess {
    let mut cg = cg.clone();
    cg.data.retain(|(a, b)| !impossible_sums.contains(&(a + b)));
    cg
}

pub fn keep_only_single_product_pairs(cg: &CurrentGuess) -> CurrentGuess {
    let mut ret = cg.clone();
    let mut product_map: HashMap<u32, u32> = HashMap::new();
    for (a, b) in cg.data.iter() {
        let product = a * b;
        *product_map.entry(product).or_default() += 1;
    }
    ret.data
        .retain(|(a, b)| product_map.get(&(a * b)).expect("logic error") == &1u32);
    ret
}

pub fn keep_only_single_sum_pairs(cg: &CurrentGuess) -> CurrentGuess {
    let mut ret = cg.clone();
    let mut sum_map: HashMap<u32, usize> = HashMap::new();
    for (a, b) in cg.data.iter() {
        let sum = a + b;
        *sum_map.entry(sum).or_default() += 1;
    }
    ret.data
        .retain(|(a, b)| sum_map.get(&(a + b)).expect("logic error") == &1);
    ret
}
