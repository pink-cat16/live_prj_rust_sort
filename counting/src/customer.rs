use crate::support;
use std::fmt;

#[derive(Clone)]
pub struct Customer {
    pub id: String,
    pub num_purchases: i32,
}

impl Customer {
    pub fn new(customer_no: i32, num_purchases: i32) -> Customer {
        Customer {
            id: format!("C{:04}", customer_no),
            num_purchases: num_purchases,
        }
    }
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.id, self.num_purchases)
    }
}


// Print at most num_items items.
pub fn print_vec(vec: &Vec<Customer>, num_items: i32) {
    // modified to indicate if whole vector is not printed
    let bigger = vec.len() > num_items as usize;
    let max = if bigger {
        num_items as usize
    } else {
        vec.len()
    };

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }

    if bigger {
        string.push_str(" ... ]");
    } else {
        string.push_str("]");
    }

    println!("{string}");
}


// separate so the sorting check can be tested.
fn is_sorted(vec: &Vec<Customer>) -> bool {
    let mut sorted: bool = true;

    for i in 1usize..vec.len() {
        if vec[i].num_purchases < vec[i  - 1usize].num_purchases {
            sorted = false;
            break;
        }
    }

    sorted
}


// prints a message if the vector is sorted or not. Vector is sorted
// when num_purchases are smallest to largest starting with index 0.
pub fn check_sorted(vec: &Vec<Customer>) {

    if is_sorted(&vec) {
        print!("The vector is sorted!");
    } else {
        print!("The vector is NOT sorted!");
    }
}


// Make a vector of random i32 values in the range [0 and max).
pub fn make_random_vec(num_items: i32, max: i32) -> Vec<Customer> {
    // Prepare a Prng.
    let mut prng = support::Prng::new();

    let mut vec: Vec<Customer> = Vec::with_capacity(num_items as usize);
    for i in 0..num_items {
        vec.push(Customer::new(i, prng.next_i32(0, max)));
    }
    return vec;
}
