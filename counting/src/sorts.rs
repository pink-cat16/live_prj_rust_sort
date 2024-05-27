use crate::customer;

pub(crate) fn count_sort(vec: &Vec<customer::Customer>, max: i32) -> Vec<customer::Customer> {
    let mut counts = vec![0; (max + 1) as usize];

    for customer in vec.iter() {
        counts[customer.num_purchases as usize] += 1;
    }

    for i in 1..counts.len() {
        counts[i] += counts[i - 1];
    }

    let mut sorted = vec![customer::Customer::new(0, 0); vec.len()];

    for i in (0..vec.len()).rev() {
        let customer = vec[i].clone();
        let index = counts[customer.num_purchases as usize] - 1;
        counts[customer.num_purchases as usize] -= 1;
        sorted[index] = customer;
    }

    return sorted;
}
