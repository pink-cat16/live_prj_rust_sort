mod support;
mod sorts;

fn main() {
    let num_items = support::get_i32("Number of items: ");
    let max =  support::get_i32("Max value: ");

    if num_items > 0 {
        let mut bubble_vec = support::make_random_vec(num_items, max);
        let mut cocktail_vec = bubble_vec.clone();

        print!("\nBubble sort pass...\n");
        support::print_vec(&bubble_vec, 30i32);
        sorts::bubble_sort(&mut bubble_vec);
        support::print_vec(&bubble_vec, 30i32);
        support::check_sorted(&bubble_vec);

        print!("\n\nCocktail shaker sort pass\n");
        support::print_vec(&cocktail_vec, 30i32);
        sorts::cocktail_sort(&mut cocktail_vec);
        support::print_vec(&cocktail_vec, 30i32);
        support::check_sorted(&cocktail_vec);

    } else {
        print!("Vector size ({}) has to be greater than zero", num_items);
    }
}
