mod sorts;
mod support;
mod customer;
use std::io;
use std::io::Write;

// Prompt the user for an i32.
fn get_i32(prompt: &str) -> i32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i32>()
        .expect("Error parsing integer");
}


fn main() {
    let num_items = get_i32("Number of items: ");
    let max =  get_i32("Max value: ");

    if num_items > 0 {
        let vec = customer::make_random_vec(num_items, max);

        customer::print_vec(&vec, 30i32);
        let sorted = sorts::count_sort(&vec, max);
        customer::print_vec(&sorted, 30i32);
        customer::check_sorted(&sorted);

    } else {
        println!("Vector size ({}) has to be greater than zero", num_items);
    }
}
