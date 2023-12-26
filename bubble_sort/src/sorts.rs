// Sort routines, for i32 vectors.


// Process vector bottom up.
fn bubble_up(vec: &mut Vec<i32>) -> bool {

    let mut sorting: bool = false;

    for i in 1usize..vec.len() {
        if vec[i] < vec[i - 1usize] {
            sorting = true;
            let swap = vec[i];
            vec[i] = vec[i - 1usize];
            vec[i - 1usize] = swap;
        }
    }

    sorting
}


// Process vector bottom up.
fn bubble_down(vec: &mut Vec<i32>) -> bool {

    let mut sorting: bool = false;

    for i in (0usize..(vec.len() - 1usize)).rev() {
        if vec[i] > vec[i + 1usize] {
            sorting = true;
            let swap = vec[i];
            vec[i] = vec[i + 1usize];
            vec[i + 1usize] = swap;
        }
    }

    sorting
}


// Use bubble sort to sort the vector.
pub(crate) fn bubble_sort(vec: &mut Vec<i32>) {

    let mut sorting: bool = true;
    let mut pass_cnt: u32 = 0;

    while sorting {
        sorting = bubble_up(vec);
        pass_cnt = pass_cnt + 1u32;
    }
    println!("Bubble pass count: {pass_cnt}");
}


// bi-directional take on the bubble sort, non-optimized version.
pub(crate) fn cocktail_sort(vec: &mut Vec<i32>) {

    let mut sorting: bool = true;
    let mut pass_cnt: u32 = 0;

    while sorting {
        sorting = bubble_up(vec);
        if sorting {
            sorting = bubble_down(vec);
            pass_cnt = pass_cnt + 1u32;
        }
        pass_cnt = pass_cnt + 1u32;
    }
    println!("cocktail pass count: {pass_cnt}");
}
