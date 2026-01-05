pub mod logger;
pub mod riddles;

use riddles as R;

// Riddle 34
// MP knows P, MS knows S
fn main() {
    crate::logger::init_logger().unwrap();
    log::info!("Starting riddles application");
    let n = 100;

    let data1 = R::initial(n);

    log::info!("Data1: len={} ", data1.data.len(),);

    // MP cannot guess from P, so we eliminate products with a single pair
    let data1 = R::remove_product_single_pairs(&data1);
    log::info!(
        "Data1 after removing single pairs: len={} ",
        data1.data.len(),
    );

    // MP says that he cannot guess, so MS knows data1
    // MS says he already knew that solution is not in ~data1,
    let complement_data1 = R::complement_guess(&data1, n);
    let impossible_sums = R::guess_sums(complement_data1);

    // MP now knows that MS could not guess, so he knows impossible_sums
    // he can eliminate pairs whose sum is in impossible_sums
    let data1 = R::remove_matching_sums(&data1, &impossible_sums);
    log::info!(
        "Data1 after removing sums known to be impossible by MS: len={} ; {:?}",
        data1.data.len(),
        data1.data,
    );

    // MP now can guess the solution from P, meaning that in data1 we keep only pairs whose product is unique
    let data1 = R::keep_only_single_product_pairs(&data1);
    log::info!(
        "Data1 after keeping only single product pairs: len={} ; {:?}",
        data1.data.len(),
        data1.data,
    );

    // MP says it, so MS knows data1, and can deduce the solution, so we only keep the pairs with a unique sum
    let data1 = R::keep_only_single_sum_pairs(&data1);
    log::info!(
        "Data1 after keeping only single sum pairs: len={} ; {:?}",
        data1.data.len(),
        data1.data,
    );
}
