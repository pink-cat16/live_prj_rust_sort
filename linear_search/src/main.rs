mod support;

// Return the target's location in the vector and the number of tests.
// If the item is not found, return -1 and the number of tests.
fn linear_search(vec: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut tests = 0i32;
    let mut location = -1i32;

    for (i, x) in vec.iter().enumerate() {
        if *x == target {
            location = i as i32;
        }
        tests += 1;                 
    }
    (location, tests)
}


fn main() {
    let num_items = support::get_i32("Number of items: ");
    let max =  support::get_i32("Max value: ");

    if num_items > 0 {
        let vec = support::make_random_vec(num_items, max);

        support::print_vec(&vec, 40i32);
        let mut target = support::get_i32("Target: ");
        while target != -1 {
            let (location, tests) = linear_search(&vec, target);
            println!("Target {} search result: {} after {} tests", target, location, tests);
            target = support::get_i32("Target: ");
        }

    } else {
        println!("Vector size ({}) has to be greater than zero", num_items);
    }
}
