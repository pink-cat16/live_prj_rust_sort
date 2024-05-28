mod support;
mod sorts;

// Perform binary search.
// Return the target's location in the vector and the number of tests.
// If the item is not found, return -1 and the number of tests.
fn binary_search(vec: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut tests: i32 = 0;
    let mut low: i32 = 0;
    let mut high: i32 = (vec.len() - 1) as i32;
    let mut location: i32 = -1;

    while low <= high {
        tests += 1;
        let mid: i32 = (low + high) / 2;
        if vec[mid as usize] < target {
            low = mid + 1;
        } else if vec[mid as usize] > target {
            high = mid - 1;
        } else {
            location = mid;
            break;
        }
    }
    (location, tests)
}

fn main() {
    let num_items = support::get_i32("Number of items: ");
    let max =  support::get_i32("Max value: ");

    if num_items > 0 {
        let mut vec = support::make_random_vec(num_items, max);
        sorts::quicksort(&mut vec);

        support::print_vec(&vec, 40i32);
        let mut target = support::get_i32("Target: ");
        while target != -1 {
            let (location, tests) = binary_search(&vec, target);
            println!("Target {} search result: {} after {} tests", target, location, tests);
            target = support::get_i32("Target: ");
        }
    } else {
        println!("Vector size ({}) has to be greater than zero", num_items);
    }
}
