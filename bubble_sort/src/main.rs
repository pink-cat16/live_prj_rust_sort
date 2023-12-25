mod support;
mod sorts;

fn main() {
    let num_items = support::get_i32("Number of items: ");
    let max =  support::get_i32("Max value: ");

    if num_items > 0 {
        let mut sample_vec = support::make_random_vec(num_items, max);
        support::print_vec(&sample_vec, 30i32);
        sorts::bubble_sort(&mut sample_vec);
        support::print_vec(&sample_vec, 30i32);
        support::check_sorted(&sample_vec);
    } else {
        print!("Vector size ({}) has to be greater than zero", num_items);
    }
}
