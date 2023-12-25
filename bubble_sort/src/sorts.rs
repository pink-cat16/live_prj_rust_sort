// Sort routines, for i32 vectors.

// Use bubble sort to sort the vector.
pub(crate) fn bubble_sort(vec: &mut Vec<i32>) {

    let mut sorting: bool = true;

    while sorting {
        sorting = false;
        for i in 1usize..vec.len() {
            if vec[i] < vec[i - 1usize] {
                sorting = true;
                let swap = vec[i];
                vec[i] = vec[i - 1usize];
                vec[i - 1usize] = swap;
            }
        }
    }
}