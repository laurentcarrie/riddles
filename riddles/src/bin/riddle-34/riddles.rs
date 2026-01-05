use std::collections::HashMap;

pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn primes_less_than_n(n: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    let mut candidate = 2;

    while candidate < n {
        if is_prime(candidate) {
            primes.push(candidate);
        }
        candidate += 1;
    }

    primes
}

pub fn sum_of_two_primes(n: u32) -> HashMap<u32, Vec<(u32, u32)>> {
    let mut sums: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for i in 2..n {
        if !is_prime(i) {
            continue;
        }
        for j in i..n {
            if !is_prime(j) {
                continue;
            }
            let sum = i + j;
            if sums.contains_key(&sum) {
                sums.get_mut(&sum).unwrap().push((i, j));
            } else {
                sums.insert(sum, vec![(i, j)]);
            }
        }
    }
    sums
}

pub fn product_of_two_numbers(n: u32) -> HashMap<u32, Vec<(u32, u32)>> {
    let mut products: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for i in 2..n {
        for j in i..n {
            let product = i * j;
            if products.contains_key(&product) {
                products.get_mut(&product).unwrap().push((i, j));
            } else {
                products.insert(product, vec![(i, j)]);
            }
        }
    }
    products
}

pub fn init_possible_values(n: u32) -> Vec<(u32, u32)> {
    let mut ret: Vec<(u32, u32)> = Vec::new();
    for i in 2..n {
        for j in 2..n {
            ret.push((i, j));
        }
    }
    ret
}

pub fn possible_products(v: Vec<(u32, u32)>) -> Vec<u32> {
    let mut products: Vec<u32> = Vec::new();
    for (a, b) in v.iter() {
        products.push(a * b);
    }
    products
}

pub fn possible_sums(v: Vec<(u32, u32)>) -> Vec<u32> {
    let mut sums: Vec<u32> = Vec::new();
    for (a, b) in v.iter() {
        sums.push(a + b);
    }
    sums
}

pub fn get_product_map(n: u32) -> HashMap<u32, Vec<(u32, u32)>> {
    let mut products: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for i in 2..n {
        for j in i..n {
            let product = i * j;
            if products.contains_key(&product) {
                products.get_mut(&product).unwrap().push((i, j));
            } else {
                products.insert(product, vec![(i, j)]);
            }
        }
    }
    products
}

pub fn get_sum_map(n: u32) -> HashMap<u32, Vec<(u32, u32)>> {
    let mut sums: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for i in 2..n {
        for j in i..n {
            let sum = i + j;
            if sums.contains_key(&sum) {
                sums.get_mut(&sum).unwrap().push((i, j));
            } else {
                sums.insert(sum, vec![(i, j)]);
            }
        }
    }
    sums
}
